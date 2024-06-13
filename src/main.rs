extern crate core;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::app::App;

mod api;
mod app;
mod gamedata;
mod legacy_login;
mod login;
mod request_structs;
mod response_structs;
#[tokio::main]
async fn main() {
	tracing_subscriber::registry()
		.with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
			|_| "axum_jwt_ware=debug,sqlx=debug,tower_http=debug,slot_car_grand_prix_server=info".into(),
		)))
		.with(tracing_subscriber::fmt::layer())
		.init();

	App::new().await.unwrap().serve().await.unwrap();
}
