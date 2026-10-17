use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = liyo_server::router();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    info!(
        "Liyo server listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.expect("HTTP server error");
}
