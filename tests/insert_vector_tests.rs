use solarpunk::{VectorData, insert_vector};
use axum::Json;
use uuid::Uuid;

#[tokio::test]
async fn test_insert_vector_success() {
    // Prepare test data with a UUID and a valid vector
    let test_data = VectorData {
        id: Uuid::new_v4(), // Generate a new UUID
        vector: vec![0.5, 0.6, 0.7],
    };

    // Send the request to insert the vector into Qdrant
    let response = insert_vector(Json(test_data)).await;

    // Check that the response matches the expected successful message
    assert_eq!(response, Ok("Vector inserted successfully".to_string()));
}

#[tokio::test]
async fn test_insert_vector_failure() {
    // Prepare test data with a UUID and an empty vector to simulate failure
    let test_data = VectorData {
        id: Uuid::new_v4(),
        vector: vec![], // Empty vector should cause a failure
    };

    // Send the request to insert the vector, expecting it to fail
    let response = insert_vector(Json(test_data)).await;

    // Assert that the response is an error, and match against the specific error message
    assert!(response.is_err());

    // Optionally, match the specific error message for confirmation
    if let Err(error_message) = response {
        assert_eq!(error_message, "Vector data cannot be empty".to_string());
    }
}
