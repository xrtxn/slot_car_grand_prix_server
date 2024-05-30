use crate::request_structs::RequestRoot;
use crate::response_structs::{
    ApiResponse, Data, LoginResponseData, Response, ResponseData, ResponseRoot,
};
use axum::Json;

pub async fn login(Json(payload): Json<RequestRoot>) -> Json<ResponseRoot> {
    println!("{:?}", payload);
    let l = ResponseRoot {
        response: Response {
            message: Some("AllOk".to_string()),
            response_data: ResponseData {
                data: Data { rows: vec![ApiResponse::LoginResponse(LoginResponseData {
                    session_id: Some(1),
                    now: Some("2024-05-28T16:00:06.106601Z".to_string()),
                })] },
            },
        },
    };
    return Json(l);
}
