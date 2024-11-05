// Imports and initial setup
use axum::{
    async_trait,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
    Server,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use neo4j::r#async::{Session, SessionConfig};  // Neo4j async session handling
use anyhow::Result;
use hyper::StatusCode;
use reqwest::Client;
use uuid::Uuid;

// Qdrant-specific structures and functions
#[derive(Serialize, Deserialize)]
pub struct VectorData {
    pub id: Uuid,
    pub vector: Vec<f32>,
}

#[derive(Serialize)]
struct QdrantPayload {
    points: Vec<VectorDataWrapper>,
}

#[derive(Serialize)]
struct VectorDataWrapper {
    id: Uuid,
    vector: Vec<f32>,
}

pub async fn insert_vector(Json(data): Json<VectorData>) -> Result<String, String> {
    if data.vector.is_empty() {
        return Err("Vector data cannot be empty".to_string());
    }

    let client = Client::new();
    let payload = QdrantPayload {
        points: vec![VectorDataWrapper {
            id: data.id,
            vector: data.vector,
        }],
    };

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

// Neo4j-specific code for user profiles
#[derive(Serialize, Deserialize)]
struct UserProfile {
    id: String,
    name: String,
    bio: String,
    location: String,
    skills: Vec<String>,
    interests: Vec<String>,
}

#[async_trait]
trait UserProfileRepository {
    async fn create(&self, profile: UserProfile) -> Result<()>;
    async fn update(&self, id: &str, profile: UserProfile) -> Result<()>;
}

struct Neo4jUserProfileRepository {
    session: Arc<Session>,
}

#[async_trait]
impl UserProfileRepository for Neo4jUserProfileRepository {
    async fn create(&self, profile: UserProfile) -> Result<()> {
        let query = "CREATE (u:User {id: $id, name: $name, bio: $bio, location: $location}) RETURN u";
        let params = neo4j::Params::from_iter([
            ("id", &profile.id),
            ("name", &profile.name),
            ("bio", &profile.bio),
            ("location", &profile.location),
        ]);
        self.session.run(query, params).await?;
        Ok(())
    }

    async fn update(&self, id: &str, profile: UserProfile) -> Result<()> {
        let query = "MATCH (u:User {id: $id}) SET u += {name: $name, bio: $bio, location: $location} RETURN u";
        let params = neo4j::Params::from_iter([
            ("id", id),
            ("name", &profile.name),
            ("bio", &profile.bio),
            ("location", &profile.location),
        ]);
        self.session.run(query, params).await?;
        Ok(())
    }
}

// Combine Qdrant and Neo4j routes in the same app
fn app(neo4j_repo: Arc<Neo4jUserProfileRepository>) -> Router {
    Router::new()
        .route("/insert_user_profile", post(insert_user_profile))
        .route("/insert_vector", post(insert_vector))
        .layer(Extension(neo4j_repo))
}

async fn insert_user_profile(
    Json(profile_data): Json<UserProfile>,
    Extension(repo): Extension<Arc<Neo4jUserProfileRepository>>,
) -> impl IntoResponse {
    match repo.create(profile_data).await {
        Ok(_) => (StatusCode::CREATED, "Profile created successfully".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)),
    }
}

// Main function with Neo4j session setup
#[tokio::main]
async fn main() {
    let neo4j_session = Session::new(&SessionConfig::default()).await.expect("Failed to create Neo4j session");
    let neo4j_repo = Arc::new(Neo4jUserProfileRepository {
        session: Arc::new(neo4j_session),
    });

    let app = app(neo4j_repo);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
