// src/lib.rs

pub mod qdrant_crud; 
pub use qdrant_crud::{VectorData, insert_vector}; 

pub mod assets;