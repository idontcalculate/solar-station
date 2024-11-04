use solarpunk::insert_vector; // Import from the library crate
use axum::{routing::post, Router};
use std::net::SocketAddr;
use axum_server::Handle;

#[tokio::main]
async fn main() {
    // Create the Axum app with routes
    let app = Router::new().route("/insert_vector", post(insert_vector));

    // Define the address and start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let handle = Handle::new();

    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
