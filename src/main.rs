use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Serve files from the static directory
    let serve_dir = ServeDir::new("static");

    // Build our application with a route
    let app = Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    println!("Server running on http://{}", addr);
    
    // Use the correct server binding method for axum 0.7
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
