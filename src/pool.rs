use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Pool, Postgres,
};
use std::time::Duration;

pub async fn get_pool() -> Pool<Postgres> {
    let options = PgConnectOptions::new()
        .host("localhost")
        .username("postgres")
        .password("postgres")
        .port(5444)
        .ssl_mode(PgSslMode::Prefer)
        .database("todo_db");

    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy_with(options)
}
