use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // construct our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    // assign a port for the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Listening on {}", addr);

    // run our app with hyper
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
