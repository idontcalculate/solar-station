use solarpunk::qdrant_crud::{insert_vector, VectorData}; // Import from the `qdrant_crud` module
use axum::Json;
use uuid::Uuid;

#[tokio::test]
async fn test_insert_vector_success() {
    // Arrange: create a valid VectorData instance
    let test_data = VectorData {
        id: Uuid::new_v4(),
        vector: vec![0.1, 0.2, 0.3],
    };

    // Act: call the insert_vector function
    let response = insert_vector(Json(test_data)).await;

    // Assert: expect a success response
    assert!(response.is_ok());
    assert_eq!(response.unwrap(), "Vector inserted successfully");
}

#[tokio::test]
async fn test_insert_vector_empty_vector() {
    // Arrange: create a VectorData instance with an empty vector
    let test_data = VectorData {
        id: Uuid::new_v4(),
        vector: vec![],
    };

    // Act: call the insert_vector function
    let response = insert_vector(Json(test_data)).await;

    // Assert: expect an error response due to empty vector
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), "Vector data cannot be empty");
}
