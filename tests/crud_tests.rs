use solarpunk::qdrant_crud::{insert_vector, VectorData}; 
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

    // Debugging: print the response for detailed inspection
    match &response {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => eprintln!("Failure: {}", err),
    }

    // Assert: expect a success response
    assert!(response.is_ok(), "Expected success, got error: {:?}", response);
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

    // Debugging: print the response for detailed inspection
    if response.is_err() {
        println!("Error received as expected: {:?}", response);
    } else {
        eprintln!("Unexpected success response: {:?}", response);
    }

    // Assert: expect an error response due to empty vector
    assert!(response.is_err(), "Expected error due to empty vector, got success");
    assert_eq!(response.unwrap_err(), "Vector data cannot be empty");
}
