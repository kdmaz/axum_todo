use async_redis_session::RedisSessionStore;
use axum::{Extension, Server};
use tower::ServiceBuilder;
mod pool;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = pool::get_pool().await;
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")?;

    let middleware = ServiceBuilder::new()
        .layer(Extension(pool))
        .layer(Extension(store));

    let app = routes::get_router().layer(middleware);

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}
