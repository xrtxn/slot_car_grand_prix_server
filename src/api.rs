use crate::gamedata::{CarBody, CarColor, GameData};
use axum::Json;

use crate::request_structs::RequestRoot;
use crate::response_structs::{
    ApiResponse, Data, GetMatchedResponseData, HighScoreResponseData, PersonalBestResponseData,
    PostScoreResponseData, Response, ResponseData, ResponseRoot,
};

pub async fn getmyscore(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    println!("{:?}", payload);
    let l = ResponseRoot {
        response: Response {
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

pub async fn gethighscore(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    println!("{:?}", payload);
    let l = ResponseRoot {
        response: Response {
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::HighScore(HighScoreResponseData {
                        user_name: "HighScoreResponseData".to_string(),
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
                           user_name: "TopPlayer2".to_string(),
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
                           user_name: "HighScoreResponseData".to_string(),
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

pub async fn postscore(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    println!("{:?}", payload);
    let l = ResponseRoot {
        response: Response {
            message: None,
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::PostScore(PostScoreResponseData {
                        in_track_id: 0,
                        place: 11,
                        score2: 25430,
                    })],
                },
            },
        },
    };
    Json(l)
}
pub async fn getmatched(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    println!("{:?}", payload);
    let l = ResponseRoot {
        response: Response {
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
