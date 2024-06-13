use axum::{Extension, Json};
use axum_jwt_ware::Claims;
use sqlx::PgPool;

use crate::gamedata::{
	CarBody, carbody_serialize, CarColor, carcolor_serialize, GameData, string_compress,
};
use crate::request_structs::{RequestData, RequestRoot};
use crate::response_structs::{
	ApiResponse, Data, GetMatchedResponseData, HighScoreResponseData, PersonalBestResponseData,
	PostScoreResponseData, Response, ResponseData, ResponseRoot,
};

pub async fn getmyscore(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    dbg!(payload);
    let l = ResponseRoot {
		response: Response {
			code: Some(0),
			message: None,
			response_data: ResponseData {
				data: Data { rows: vec![ApiResponse::PersonalBest(PersonalBestResponseData {
					user_name: "GetPersonalBest".to_string(),
					opt_login_id: -1,
					track_id: 0,
					place: 69,
					my_best_time: 26378,
					game_data: GameData {
						version: 91,
						car_body: CarBody::Standard,
						car_color: CarColor::Red,
						data: "2000000111111111111111111111111111111111110000000000000011111111110000000001111111111111111111111111111111111111111111110000000000000111111111000011111111111111111111111111111111111111111100000000011111100001111110000111111111111111111111111111000000000000000011110000000001110001111111111111111111111100000000000111111111111111111000000001111111111111111111111111111111111111111111111111111111111111111111111111111111111111000000000000000000011111111000000000111110011110000001111111111111111111111111111111111111111100000000000111111111111111111111111111111111111111100000000000111111111100000000000111111111111111111111111111111111000000000000000000011111111111111111111111111111111111110000000000000000011111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
					},
					last_place: 100,
				})] },
			},
		},
	};
    Json(l)
}

pub async fn gethighscore(
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

	let l = ResponseRoot {
		response: Response {
			code: Some(0),
			message: None,
			response_data: ResponseData {
				data: Data {
					rows: vec![]
				}
			}
		}
	};


    let l = ResponseRoot {
		response: Response {
			code: Some(0),
			message: None,
			response_data: ResponseData {
				data: Data {
					rows: vec![ApiResponse::HighScore(HighScoreResponseData {
						username: "HighScoreResponseData".to_string(),
						score2: 25430,
						track_id: 0,
						rank: 1,
						color: 1,
						opt_login_id: -1,
						game_data: GameData {
							version: 91,
							car_body: CarBody::Standard,
							car_color: CarColor::Red,
							data: "01000011111111111111111111111111111111111111000011100011111111111111111111111111111111111111111111111111111111100011111111111111111111111111111111111111111111110000000001111111111111111111111111110000000000000000011111111111111111111111111111111000011100000000000011111111111111111111111111111111111111111111111111111111111111111111111111111111111110000000011100000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111100000000111111111111111111111111111111100000000001111111111111111111111111111111111111111111111111111111111111111111000011111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
						},
					}
					),
						ApiResponse::HighScore(HighScoreResponseData {
						username: "TopPlayer2".to_string(),
						score2: 26378,
						track_id: 0,
						rank: 2,
						color: 1,
						opt_login_id: -1,
						game_data: GameData{
						version: 91,
						car_body: CarBody::Standard,
						car_color: CarColor::Red,
						data: "2000000111111111111111111111111111111111110000000000000011111111110000000001111111111111111111111111111111111111111111110000000000000111111111000011111111111111111111111111111111111111111100000000011111100001111110000111111111111111111111111111000000000000000011110000000001110001111111111111111111111100000000000111111111111111111000000001111111111111111111111111111111111111111111111111111111111111111111111111111111111111000000000000000000011111111000000000111110011110000001111111111111111111111111111111111111111100000000000111111111111111111111111111111111111111100000000000111111111100000000000111111111111111111111111111111111000000000000000000011111111111111111111111111111111111110000000000000000011111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
						} ,
						}
						),
						ApiResponse::HighScore(HighScoreResponseData {
						username: "HighScoreResponseData".to_string(),
						score2: 26841,
						track_id: 0,
						rank: 3,
						color: 1,
						opt_login_id: -1,
						game_data: GameData {
						version: 91,
						car_body: CarBody::Standard,
						car_color: CarColor::Red,
						data: "0101111111111111000000000000001111111111111111111111111111111111111111111111111111111111111111111111111111111111111111000011111111111111111111111111111111111111111110000111111111111111111111111111111110000000000000000011111111111111111111111111000001111100000000000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111000000000000000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111000000000000000111111111111111111111100000000000000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string()
						},
						}
						)
					],
				},
			},
		},
	};
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
        INSERT INTO data.score (userid, build, track_id, timescore, carbody_id, car_color, data)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id
        "#,
        claims.sub.parse::<i32>().unwrap(),
        request.gamedata.version as i16,
        request.track_id as i16,
        request.score2 as i32,
        carbody_serialize(&request.gamedata.car_body),
        carcolor_serialize(&request.gamedata.car_color),
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
                        place: get_time_id_rank(pool.0, db_id.id).await,
                        score2: request.score2,
                    })],
                },
            },
        },
    };
    Json(l)
}

//todo check with trackid
async fn get_time_id_rank(pg_pool: PgPool, id: i32) -> u64 {
    sqlx::query!(
        r#"
        SELECT COUNT(*)+1 AS rank
        FROM data.score WHERE timescore<(
        SELECT timescore FROM data.score
        WHERE data.score.id = $1);
        "#,
        id
    )
    .fetch_one(&pg_pool)
    .await
    .expect("TODO: panic message")
    .rank
    .unwrap() as u64
}

pub async fn getmatched(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    dbg!(payload);
    let l = ResponseRoot {
		response: Response {
			code: Some(0),
			message: None,
			response_data: ResponseData {
				data: Data {
					rows: vec![ApiResponse::GetMatched(GetMatchedResponseData {
						user_name: "Layosss".to_string(),
						opt_login_id: -1,
						track_id: 0,
						place: 11,
						score2: 25430,
						game_data: GameData {
							version: 91,
							car_body: CarBody::Standard,
							car_color: CarColor::Red,
							data: "01000011111111111111111111111111111111111111000011100011111111111111111111111111111111111111111111111111111111100011111111111111111111111111111111111111111111110000000001111111111111111111111111110000000000000000011111111111111111111111111111111000011100000000000011111111111111111111111111111111111111111111111111111111111111111111111111111111111110000000011100000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111100000000111111111111111111111111111111100000000001111111111111111111111111111111111111111111111111111111111111111111000011111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
						},
						last_place: 612,
					})],
				},
			},
		},
	};
    Json(l)
}
