use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum_server::Handle;

#[derive(Serialize, Deserialize)]
struct VectorData {
    id: String,
    vector: Vec<f32>,
}

async fn insert_vector(Json(_data): Json<VectorData>) -> Result<String, String> {
    Ok("Vector inserted successfully".to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/insert_vector", post(insert_vector));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let handle = Handle::new();

    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
