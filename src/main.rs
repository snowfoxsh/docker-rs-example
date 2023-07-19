use std::net::SocketAddr;
use axum::{Router, Server, ServiceExt};
use axum::routing::get;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let route: Router<_, _> = Router::new()
        .route("/", get(root));

    println!("http://localhost:8080");
    Server::bind(&addr)
        .serve(route.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

