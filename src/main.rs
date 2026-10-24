mod app;
pub mod errors;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = app::create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed bind listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start serve")
}
