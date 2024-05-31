extern crate core;

mod api;
mod gamedata;
mod login;
mod request_structs;
mod response_structs;
use crate::api::{gethighscore, getmyscore};
use crate::api::{getmatched, postscore};
use crate::login::login;
use axum::extract::Query;
use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use tracing::debug;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/LocalTestServer/MongoBoxServlet/login", post(login))
        .route(
            "/LocalTestServer/MongoBoxServlet/getmyscore",
            post(getmyscore),
        )
        .route(
            "/LocalTestServer/MongoBoxServlet/gethighscore",
            post(gethighscore),
        )
        .route(
            "/LocalTestServer/MongoBoxServlet/postscore",
            post(postscore),
        )
        .route(
            "/LocalTestServer/MongoBoxServlet/getmatched",
            post(getmatched),
        )
        .route(
            "/LocalTestServer/MongoBoxServlet/*action",
            get(unimplemented),
        );

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// This get's called when an as3 function was not rewritten
async fn unimplemented(Query(params): Query<HashMap<String, String>>) -> &'static str {
    debug!("Function not yet written for: {:?}", params);
    "Not yet implemented"
}
