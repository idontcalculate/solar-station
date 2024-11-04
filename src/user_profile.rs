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
use neo4j::r#async::{Session, SessionConfig};  // Adjusted for correct async session handling
use qdrant_client::QdrantClient;
use anyhow::Result;  // Ensure anyhow is in your Cargo.toml

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
    // Additional methods like delete, find_by_id, etc.
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

async fn insert_user_profile(
    Json(profile_data): Json<UserProfile>,
    Extension(repo): Extension<Arc<Neo4jUserProfileRepository>>,
) -> impl IntoResponse {
    match repo.create(profile_data).await {
        Ok(_) => (StatusCode::CREATED, "Profile created successfully".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)),
    }
}

fn app(neo4j_repo: Arc<Neo4jUserProfileRepository>) -> Router {
    Router::new()
        .route("/insert_user_profile", post(insert_user_profile))
        .layer(Extension(neo4j_repo))
}

#[tokio::main]
async fn main() {
    let neo4j_session = Session::new(&SessionConfig::default()).await.unwrap();  // Ensures async session creation
    let neo4j_repo = Arc::new(Neo4jUserProfileRepository {
        session: Arc::new(neo4j_session),
    });

    let app = app(neo4j_repo);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
