use axum::Json;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct VectorData {
    pub id: Uuid,
    pub vector: Vec<f32>,
}

// Structure matching Qdrant's expected payload format
#[derive(Serialize)]
struct QdrantPayload {
    points: Vec<VectorDataWrapper>,
    ids: Vec<Uuid>, // Explicitly include `ids` for each point
}

// Wrapper structure for each point, as required by Qdrant
#[derive(Serialize)]
struct VectorDataWrapper {
    vector: Vec<f32>,
}

// Route handler to insert vector into Qdrant
pub async fn insert_vector(Json(data): Json<VectorData>) -> Result<String, String> {
    // Check if the vector is empty and return an error if so
    if data.vector.is_empty() {
        return Err("Vector data cannot be empty".to_string());
    }

    let client = Client::new();

    // Wrap the data into Qdrant's expected payload structure
    let payload = QdrantPayload {
        points: vec![VectorDataWrapper {
            vector: data.vector,
        }],
        ids: vec![data.id],
    };

    // Send request to Qdrant
    let response = client
        .post("http://localhost:6333/collections/solar-collection/points")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to Qdrant: {}", e))?;

    if response.status().is_success() {
        Ok("Vector inserted successfully".to_string())
    } else {
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to insert vector: {}", error_message))
    }
}
