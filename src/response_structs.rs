use serde::{Deserialize, Serialize};
use crate::gamedata::GameData;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Login(LoginResponseData),
    PersonalBest(PersonalBestResponseData),
    HighScore(HighScoreResponseData),
    PostScore(PostScoreResponseData),
    GetMatched(GetMatchedResponseData),
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Row")]
    pub rows: Vec<ApiResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseData")]
    pub response_data: ResponseData,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseRoot {
    pub response: Response,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponseData {
    #[serde(rename = "sessionID")]
    //todo u32
    pub session_id: Option<i32>,
    pub now: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PersonalBestResponseData {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub place: i64,
    #[serde(rename = "my_best_time")]
    pub my_best_time: i64,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
    #[serde(rename = "lastPlace")]
    pub last_place: u64,
}

#[derive(Serialize, Deserialize)]
pub struct HighScoreResponseData {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub score2: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub rank: i64,
    pub color: i64,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Serialize, Deserialize)]
pub struct PostScoreResponseData {
    #[serde(rename = "IN_trackID")]
    pub in_track_id: i64,
    pub place: i64,
    pub score2: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetMatchedResponseData {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub place: i64,
    pub score2: i64,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
    #[serde(rename = "lastPlace")]
    pub last_place: i64,
}
