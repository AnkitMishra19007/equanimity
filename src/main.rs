use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(handler));

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Basic handler that responds with a static string
async fn handler() -> &'static str {
    "Hello, World!"
}
