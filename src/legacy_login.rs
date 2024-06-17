use crate::request_structs::RequestRoot;
use crate::response_structs::{
    ApiResponse, Data, LegacyLoginResponseData, Response, ResponseData, ResponseRoot,
};
use axum::Json;

#[deprecated(since = "0.1.1", note = "This is done client side only")]
#[allow(dead_code)]
pub async fn legacy_login(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    dbg!(payload);
    let l = ResponseRoot {
        response: Response {
            code: Some(0),
            message: Some("AllOk".to_string()),
            response_data: ResponseData {
                data: Data {
                    rows: vec![ApiResponse::LegacyLoginResponse(LegacyLoginResponseData {
                        session_id: Some(-1),
                        now: Some("2024-05-28T17:00:06.106601Z".to_string()),
                    })],
                },
            },
        },
    };
    Json(l)
}
