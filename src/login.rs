use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_jwt_ware::{
    auth_token_decode, auth_token_encode, Claims, DecodingContext, DecodingKey, EncodingContext,
    EncodingKey, Header,
};
use chrono::{Duration, Utc};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::request_structs::{LoginRequest, RefreshRequest, RequestData, RequestRoot};
use crate::response_structs::{
    ApiResponse, Data, LegacyLoginResponseData, RegisterResponseData, Response, ResponseData,
    ResponseRoot,
};

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQLUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub async fn full_login(pool: Extension<PgPool>, body: Json<LoginRequest>) -> impl IntoResponse {
    let request_data = User {
        username: body.username.clone(),
        password: body.password.clone(),
    };

    //todo handle not existing user
    let sql_user = sqlx::query_as!(
        SQLUser,
        "SELECT * FROM data.users WHERE username LIKE $1",
        request_data.username
    )
    .fetch_optional(&pool.0)
    .await
    .unwrap()
    .unwrap();

    match verify_password(request_data.password, &sql_user.password) {
        Ok(_) => println!(
            "Password succesfully verified for {}!",
            request_data.username
        ),
        Err(_) => {
            println!("Wrong password for {}!", request_data.username);
            let error = AuthError {
                message: "Invalid username or password".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            };
            return Err(error);
        }
    };

    let key = EncodingKey::from_secret(
        dotenvy::var("JWT_SECRET")
            .expect("JWT_SECRET not set!")
            .as_bytes(),
    );
    let refresh_key = EncodingKey::from_secret(
        dotenvy::var("JWT_REFRESH_SECRET")
            .expect("JWT_REFRESH_SECRET not set!")
            .as_bytes(),
    );
    let header = &Header::default();
    let refresh_header = &Header::default();
    let expiry_timestamp = Utc::now() + Duration::hours(48);

    let claims = Claims {
        sub: sql_user.id.to_string(),
        username: request_data.username.clone(),
        exp: expiry_timestamp.timestamp(),
    };

    let access_token = auth_token_encode(claims.clone(), header, &key).await;
    let refresh_token = auth_token_encode(claims, refresh_header, &refresh_key).await;
    let response = Json(json!({
        "access_token": access_token.expect("Invalid token"),
        "refresh_token": refresh_token.expect("invalid refresh token")
    }));
    Ok(response)
}

pub async fn register(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenvy::var("DATABASE_URL").expect("Env variable DATABASE_URL not set!"))
        .await
        .unwrap();

    let resp = match payload.request_data {
        RequestData::Register(reg) => reg,
        _ => {
            return Json(ResponseRoot {
                response: Response {
                    // Failed to parse register request
                    code: Some(1),
                    message: None,
                    response_data: ResponseData {
                        data: Data { rows: vec![] },
                    },
                },
            });
        }
    };

    let exists = sqlx::query!(
        r#"
        SELECT username
        FROM data.users
        WHERE username LIKE $1 ;
        "#,
        resp.username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match exists {
        None => {}
        Some(_) => {
            return Json(ResponseRoot {
                response: Response {
                    // username already exists
                    code: Some(1),
                    message: None,
                    response_data: ResponseData {
                        data: Data {
                            rows: vec![ApiResponse::LegacyLoginResponse(LegacyLoginResponseData {
                                session_id: None,
                                now: None,
                            })],
                        },
                    },
                },
            });
        }
    }

    sqlx::query!(
        r#"
        INSERT INTO data.users (username, password)
        VALUES ($1, $2)
        "#,
        resp.username,
        password_auth::generate_hash(resp.password),
    )
    .execute(&pool)
    .await
    .unwrap();

    let l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::RegisterResponse(RegisterResponseData {
                        username: resp.username,
                    })],
                },
            },
        },
    };
    Json(l)
}

pub async fn refresh(body: Json<RefreshRequest>) -> impl IntoResponse {
    let encoding_context = EncodingContext {
        header: Header::default(),
        validation: axum_jwt_ware::Validation::default(),
        key: EncodingKey::from_secret(
            dotenvy::var("JWT_SECRET")
                .expect("JWT_SECRET not set!")
                .as_bytes(),
        ),
    };
    let decoding_context = DecodingContext {
        header: Header::default(),
        validation: axum_jwt_ware::Validation::default(),
        key: DecodingKey::from_secret(
            dotenvy::var("JWT_REFRESH_SECRET")
                .expect("JWT_REFRESH_SECRET not set!")
                .as_bytes(),
        ),
    };

    refresh_token(body, encoding_context, decoding_context, None).await
}

pub async fn refresh_token(
    body: Json<RefreshRequest>,
    encoding_context: EncodingContext,
    decoding_context: DecodingContext,
    new_claims: Option<Claims>,
) -> impl IntoResponse {
    let token = &body.refresh_token;

    match auth_token_decode(
        token.to_string(),
        &decoding_context.key,
        decoding_context.validation,
    )
    .await
    {
        Ok(mut claims) => {
            if let Some(new) = new_claims {
                claims.claims = new
            }
            claims.claims.exp = (Utc::now() + Duration::hours(48)).timestamp();
            match auth_token_encode(
                claims.claims,
                &encoding_context.header,
                &encoding_context.key,
            )
            .await
            {
                Ok(new_token) => Ok(Json(json!({"access_token": new_token}))),
                Err(_) => Err(AuthError {
                    message: "Invalid refresh token".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                }),
            }
        }

        Err(_) => Err(AuthError {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }),
    }
}
