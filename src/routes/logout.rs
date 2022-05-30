use async_redis_session::RedisSessionStore;
use async_session::SessionStore;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use cookie::{Cookie, CookieJar, Key};

pub async fn post(Extension(store): Extension<RedisSessionStore>, headers: HeaderMap) -> Response {
    let cookie_header = match headers.get("cookie") {
        Some(cookie_header) => cookie_header,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };
    let cookie_header = match cookie_header.to_str() {
        Ok(cookie_header) => cookie_header,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    if destory_session_from_cookie(cookie_header.to_owned(), store)
        .await
        .is_err()
    {
        StatusCode::BAD_REQUEST.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}

async fn destory_session_from_cookie(
    cookie_header: String,
    store: RedisSessionStore,
) -> Result<(), anyhow::Error> {
    let key =
        Key::from("super-long-and-secret-random-key-needed-to-verify-message-integrity".as_bytes());
    let cookie = Cookie::parse(cookie_header)?;
    let mut jar = CookieJar::new();
    jar.add_original(cookie);
    let cookie = jar
        .private(&key)
        .get("id")
        .ok_or_else(|| anyhow::anyhow!("Failed to parse cookie"))?;

    let session = store
        .load_session(cookie.value().to_owned())
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get session"))?;

    store.destroy_session(session).await?;

    Ok(())
}
