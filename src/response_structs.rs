use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResponse {
    RegisterResponse(RegisterResponseData),
    LoginResponse(LoginResponseData),
    PersonalBest(PersonalBestResponseData),
    HighScore(HighScoreResponseData),
    PostScore(PostScoreResponseData),
    GetMatched(GetMatchedResponseData),
    #[deprecated(since = "0.1.1", note = "This is done client side only")]
    LegacyLoginResponse(LegacyLoginResponseData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "Row")]
    pub rows: Vec<ApiResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseData")]
    pub response_data: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRoot {
    pub response: Response,
}

//todo tmp
impl Default for ResponseRoot {
    fn default() -> Self {
        ResponseRoot {
            response: Response {
                code: Some(1),
                message: None,
                response_data: ResponseData {
                    data: Data { rows: vec![] },
                },
            },
        }
    }
}

#[deprecated(since = "0.1.1", note = "This is done client side only")]
#[derive(Serialize, Deserialize, Debug)]
pub struct LegacyLoginResponseData {
    #[serde(rename = "sessionID")]
    pub session_id: Option<i32>,
    pub now: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalBestResponseData {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub place: i64,
    #[serde(rename = "myBestTime")]
    pub my_best_time: i64,
    #[serde(rename = "gameData")]
    pub game_data: String,
    #[serde(rename = "lastPlace")]
    pub last_place: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighScoreResponseData {
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "score2")]
    pub timescore: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub rank: i64,
    #[serde(rename = "color")]
    pub car_color: i64,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "gameData")]
    pub game_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostScoreResponseData {
    #[serde(rename = "IN_trackID")]
    pub in_track_id: i64,
    pub place: u64,
    pub score2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMatchedResponseData {
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "optLoginID")]
    pub opt_login_id: i64,
    #[serde(rename = "trackID")]
    pub track_id: i64,
    pub place: i64,
    pub score2: i64,
    #[serde(rename = "gameData")]
    pub game_data: String,
    #[serde(rename = "lastPlace")]
    pub last_place: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponseData {
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponseData {
    pub token: String,
}
