use axum::{Extension, Server};
use tower::ServiceBuilder;
mod pool;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = pool::get_pool().await;

    let middleware = ServiceBuilder::new().layer(Extension(pool));

    let app = routes::get_router().layer(middleware);

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}
