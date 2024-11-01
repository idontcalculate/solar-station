use axum::{Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;  // For making HTTP requests to Qdrant

#[derive(Serialize, Deserialize)]
pub struct VectorData {
    pub id: String,
    pub vector: Vec<f32>,
}

// Route handler to insert vector into Qdrant
pub async fn insert_vector(Json(data): Json<VectorData>) -> Result<String, String> {
    let client = Client::new();

    // Replace the collection name with 'solar-collection'
    let response = client.post("http://qdrant:6333/collections/solar-collection/points")
        .json(&data)
        .send()
        .await
        .map_err(|e| format!("Failed to insert vector: {}", e))?;

    if response.status().is_success() {
        Ok("Vector inserted successfully".to_string())
    } else {
        Err("Failed to insert vector".to_string())
    }
}
