use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use axum::{State, Json, http::StatusCode};

#[derive(Serialize, Deserialize)]
struct HelloWorldRequest {
    message: String
}

async fn hello_world(
    Json(req): Json<HelloWorldRequest>
) -> impl IntoResponse {
    
    (StatusCode::OK, req.message)
}

async fn heartbeat() -> &'static str {
    "server online"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(heartbeat));

    Ok(router.into())
}
