use axum::{Router};
use std::net::SocketAddr;
use axum_server::Handle;
use tokio::signal;

#[tokio::main]
async fn main() {
    // Create the Axum app without routes for now
    let app = Router::new();

    // Define the server address and instantiate the Handle for shutdown management
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let handle = Handle::new();

    println!("Server running at http://{}", addr);

    // Start the Axum server
    let server = axum_server::bind(addr)
        .handle(handle.clone())
        .serve(app.into_make_service());

    // Graceful shutdown on interrupt signal (Ctrl+C)
    let graceful = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
        handle.shutdown();
    };

    tokio::select! {
        _ = server => {},
        _ = graceful => {
            println!("Server shutting down gracefully.");
        },
    }
}
