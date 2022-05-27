use axum::{Extension, Server};
mod pool;
mod routes;

#[tokio::main]
async fn main() {
    let pool = pool::get_pool().await;
    let app = routes::get_router().layer(Extension(pool));

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server")
}
