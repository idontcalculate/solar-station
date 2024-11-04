// src/lib.rs

// Make qdrant_routes module public so other parts of the crate can access it.
pub mod qdrant_routes;

// Re-export the VectorData struct and insert_vector function from qdrant_routes
// so they can be easily accessed when importing the crate.
pub use qdrant_routes::{VectorData, insert_vector};
