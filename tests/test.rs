#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json};
    use serde_json::json;

    #[tokio::test]
    async fn test_insert_vector_success() {
        let test_data = VectorData {
            id: "vector_test".to_string(),
            vector: vec![0.5, 0.6, 0.7],
        };
        let response = insert_vector(Json(test_data)).await;

        assert_eq!(response, Ok("Vector inserted successfully".to_string()));
    }

    #[tokio::test]
    async fn test_insert_vector_failure() {
        let test_data = VectorData {
            id: "".to_string(),  // Example of invalid data
            vector: vec![],
        };
        let response = insert_vector(Json(test_data)).await;

        assert!(response.is_err());
    }
}
