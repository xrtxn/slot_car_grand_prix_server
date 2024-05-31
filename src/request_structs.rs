use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use crate::gamedata::GameData;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRoot {
    pub checksum: String,
    #[serde(rename = "requestdata")]
    pub request_data: RequestData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum RequestData {
    #[serde(rename = "User.Login")]
    Login(LoginRequestData),
    #[serde(rename = "AppScore.GetPersonalBest")]
    GetMyScore(GetMyScoreRequestData),
    #[serde(rename = "AppScore.GetHighScore")]
    GetHighScore(GetHighScoreData),
    #[serde(rename = "AppScore.SetScore")]
    PostScore(PostScore),
    #[serde(rename = "AppScore.GetMatched")]
    GetMatched(GetMatchedData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestData {
    #[serde(rename = "appid")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u8,
    pub username: String,
    #[serde(rename = "userlocation")]
    pub user_location: String,
    #[serde(rename = "userpassword")]
    pub user_password: String,
    #[serde(rename = "useremail")]
    pub user_email: String,
    #[serde(rename = "optloginid")]
    pub opt_login_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetMyScoreRequestData {
    #[serde(rename = "appid")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u8,
    #[serde(rename = "byscore")]
    pub by_score: u8,
    #[serde(rename = "sessionid")]
    pub session_id: String,
    #[serde(rename = "trackid")]
    pub track_id: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetHighScoreData {
    #[serde(rename = "appid")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u8,
    #[serde(rename = "sortbyscore")]
    pub sort_by_score: u8,
    #[serde(rename = "timeframe")]
    pub time_frame: String,
    #[serde(rename = "timeid")]
    pub time_id: u8,
    #[serde(rename = "trackid")]
    pub track_id: u8,
    #[serde(rename = "maxcount")]
    pub max_count: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostScore {
    #[serde(rename = "buildversion")]
    pub build_version: i64,
    #[serde(rename = "appid")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub app_id: u8,
    #[serde(rename = "sessionid")]
    pub session_id: String,
    #[serde(rename = "trackid")]
    pub track_id: u8,
    pub score1: u64,
    pub score2: u64,
    pub score3: u64,
    pub score4: u64,
    pub gamedata: GameData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMatchedData {
    pub score: u64,
    #[serde(rename = "trackid")]
    pub track_id: u8,
    #[serde(rename = "sessionid")]
    pub session_id: String,
}
