use axum::{Extension, Json};
use axum_jwt_ware::Claims;
use chrono::Utc;
use sqlx::PgPool;

use crate::gamedata::{
    carbody_serialize, carcolor_serialize, data_field_decompress, string_compress,
};
use crate::request_structs::{RequestData, RequestRoot};
use crate::response_structs::{
    ApiResponse, Data, GetMatchedResponseData, HighScoreResponseData, PersonalBestResponseData,
    PostScoreResponseData, Response, ResponseData, ResponseRoot,
};

pub async fn getmyscore(
    pool: Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RequestRoot>,
) -> Json<ResponseRoot> {
    let request = match payload.request_data {
        RequestData::GetMyScore(post) => post,
        _ => {
            return Json(ResponseRoot::default());
        }
    };

    let db_resp = sqlx::query!(
        "SELECT
		score.id as score_id,
        score.userid,
        score.track_id,
        score.timescore,
        u.id as user_id,
        u.username,
        build,
        carbody_id,
        car_color,
        data
		FROM data.score INNER JOIN data.users u on u.id = score.userid
		WHERE track_id = $1 AND u.id = $2 ORDER BY timescore LIMIT 1;",
        request.track_id as i16,
        claims.sub.parse::<i32>().unwrap()
    )
    .fetch_one(&pool.0)
    .await
    .unwrap();

    let data_str = format!(
        "{},{},{},{}",
        db_resp.build,
        db_resp.carbody_id,
        db_resp.car_color,
        data_field_decompress(&db_resp.data).unwrap()
    );

    let last_place = get_last_place(&pool, request.track_id as i16).await;

    let l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::PersonalBest(PersonalBestResponseData {
                        user_name: db_resp.username,
                        opt_login_id: -1,
                        track_id: db_resp.track_id as i64,
                        place: get_time_id_rank(&pool.0, request.track_id as i16, db_resp.score_id)
                            .await
                            .unwrap(),
                        my_best_time: db_resp.timescore as i64,
                        game_data: string_compress(&data_str).expect("Failed to compress string"),
                        last_place: last_place.unwrap() as u64,
                    })],
                },
            },
        },
    };
    Json(l)
}

pub async fn gethighscore(
    pool: Extension<PgPool>,
    Json(payload): Json<RequestRoot>,
) -> Json<ResponseRoot> {
    let request = match payload.request_data {
        RequestData::GetHighScore(post) => post,
        _ => {
            return Json(ResponseRoot::default());
        }
    };

    let mut l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: None,
            response_data: ResponseData {
                data: Data { rows: vec![] },
            },
        },
    };

    let db_resp = sqlx::query!(
        r#"
        SELECT build, carbody_id, car_color, data, username, timescore, track_id, dense_rank() OVER (ORDER BY timescore) AS rank
        FROM data.score
        INNER JOIN data.users u on u.id = score.userid
        WHERE track_id = $1
        ORDER BY timescore, datetime
        LIMIT 8;
        "#,
        request.track_id as i16
    )
		.fetch_all(&pool.0)
		.await
		.unwrap();

    for resp in db_resp {
        let data_str = format!(
            "{},{},{},{}",
            resp.build,
            resp.carbody_id,
            resp.car_color,
            data_field_decompress(&resp.data).unwrap()
        );

        let parsed = ApiResponse::HighScore(HighScoreResponseData {
            username: resp.username,
            timescore: resp.timescore as i64,
            track_id: resp.track_id as i64,
            rank: resp.rank.unwrap(),
            car_color: resp.car_color as i64,
            opt_login_id: -1,
            game_data: string_compress(&data_str).unwrap(),
        });
        l.response.response_data.data.rows.push(parsed);
    }
    Json(l)
}

pub async fn postscore(
    pool: Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RequestRoot>,
) -> Json<ResponseRoot> {
    let request = match payload.request_data {
        RequestData::PostScore(post) => post,
        _ => {
            return Json(ResponseRoot::default());
        }
    };

    let db_id = sqlx::query!(
        r#"
        INSERT INTO data.score (userid, build, track_id, timescore, carbody_id, car_color, datetime, data)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id
        "#,
        claims.sub.parse::<i32>().unwrap(),
        request.gamedata.version as i16,
        request.track_id as i16,
        request.score2 as i32,
        carbody_serialize(&request.gamedata.car_body),
        carcolor_serialize(&request.gamedata.car_color),
		Utc::now().timestamp() as i32,
        string_compress(&request.gamedata.data).expect("Failed to compress string"),
    )
		.fetch_one(&pool.0)
		.await
		.unwrap();

    let l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::PostScore(PostScoreResponseData {
                        in_track_id: request.track_id as i64,
                        place: get_time_id_rank(&pool.0, request.track_id as i16, db_id.id)
                            .await
                            .unwrap() as u64,
                        score2: request.score2,
                    })],
                },
            },
        },
    };
    Json(l)
}

pub async fn getmatched(
    pool: Extension<PgPool>,
    Json(payload): Json<RequestRoot>,
) -> Json<ResponseRoot> {
    let request = match payload.request_data {
        RequestData::GetMatched(post) => post,
        _ => {
            return Json(ResponseRoot::default());
        }
    };

    //this gets the current record and uses it if it is the best result
    let db_resp = sqlx::query!(
        r#"
		SELECT
		score.id as score_id,
        score.userid,
        score.track_id,
        score.timescore,
        u.id as user_id,
        u.username,
        build,
        carbody_id,
        car_color,
        data,
        dense_rank() OVER (ORDER BY timescore) AS rank
        FROM data.score INNER JOIN data.users u on u.id = score.userid
		WHERE track_id = $1 AND $2 > timescore ORDER BY timescore DESC LIMIT 1;
        "#,
        request.track_id as i16,
        request.score as i32
    )
    .fetch_all(&pool.0)
    .await
    .unwrap();

    let matched_result = match db_resp.first() {
        None => return Json(ResponseRoot::default()),
        Some(matched) => matched,
    };

    let last_place = get_last_place(&pool, request.track_id as i16).await;

    let data_str = format!(
        "{},{},{},{}",
        matched_result.build,
        matched_result.carbody_id,
        matched_result.car_color,
        data_field_decompress(&matched_result.data).unwrap()
    );

    let l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::GetMatched(GetMatchedResponseData {
                        username: matched_result.username.clone(),
                        opt_login_id: -1,
                        track_id: matched_result.track_id as i64,
                        place: matched_result.rank.unwrap(),
                        score2: matched_result.timescore as i64,
                        game_data: string_compress(&data_str).expect("Failed to compress string"),
                        last_place: last_place.unwrap(),
                    })],
                },
            },
        },
    };
    Json(l)
}

async fn get_last_place(pool: &PgPool, track_id: i16) -> Option<i64> {
    sqlx::query!(
        r#"
        SELECT dense_rank() OVER (ORDER BY timescore) AS rank
        FROM data.score
        WHERE track_id = $1
        ORDER BY timescore DESC, datetime DESC
        LIMIT 1;
        "#,
        track_id as i16,
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .rank
}

async fn get_time_id_rank(pool: &PgPool, track_id: i16, score_id: i32) -> Option<i64> {
    sqlx::query!(
        r#"
        SELECT * FROM (SELECT score.id, dense_rank() OVER (ORDER BY timescore) AS rank
               FROM data.score
               WHERE track_id = $1
               GROUP BY timescore, score.id) as ir WHERE ir.id = $2;
        "#,
        track_id as i16,
        score_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .rank
}
