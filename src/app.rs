use axum::extract::Request;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router};
use axum_jwt_ware::{DecodingKey, Validation};
use sqlx::postgres::PgPool;

use crate::api::{gethighscore, getmatched, getmyscore, postscore};
use crate::login::{full_login, refresh, register};

pub struct App {
    db: PgPool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = PgPool::connect(&dotenvy::var("DATABASE_URL").expect("DATABASE_URL not defined!"))
            .await?;
        sqlx::migrate!().run(&db).await?;
        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/LocalTestServer/MongoBoxServlet/refresh", post(refresh))
            .route("/LocalTestServer/MongoBoxServlet/login", post(full_login))
            .route("/LocalTestServer/MongoBoxServlet/register", post(register))
            .route(
                "/LocalTestServer/MongoBoxServlet/getmyscore",
                post(getmyscore).layer(middleware::from_fn(jwt_access_middleware)),
            )
            .route(
                "/LocalTestServer/MongoBoxServlet/gethighscore",
                post(gethighscore).layer(middleware::from_fn(jwt_access_middleware)),
            )
            .route(
                "/LocalTestServer/MongoBoxServlet/postscore",
                post(postscore).layer(middleware::from_fn(jwt_access_middleware)),
            )
            .route(
                "/LocalTestServer/MongoBoxServlet/getmatched",
                post(getmatched),
            )
            .layer(Extension(self.db.clone()));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();

        axum::serve(listener, app.into_make_service()).await?;
        Ok(())
    }
}

async fn jwt_access_middleware(request: Request, next: middleware::Next) -> impl IntoResponse {
    let key = DecodingKey::from_secret(
        dotenvy::var("JWT_SECRET")
            .expect("JWT_SECRET not set!")
            .as_bytes(),
    );
    let validation = Validation::default();
    axum_jwt_ware::verify_user(request, &key, validation, next).await
}
