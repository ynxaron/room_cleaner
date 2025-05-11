mod models;
mod scheduler;

use axum::{routing::post, Json, Router};
use models::{Room, Task};
use scheduler::schedule_tasks;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/schedule", post(handle_schedule))
        .layer(ServiceBuilder::new().layer(CorsLayer::very_permissive()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Backend running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_schedule(Json(rooms): Json<Vec<Room>>) -> Json<Vec<Task>> {
    let tasks = schedule_tasks(rooms);
    Json(tasks)
}
