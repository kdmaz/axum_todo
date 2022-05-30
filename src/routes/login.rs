use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Extension, Json,
};
use cookie::{time::Duration, Cookie, CookieJar, Key};
use secrecy::Secret;
use sqlx::{query, PgPool};
use todo::{
    auth::{validate_user, Credentials},
    user::User,
};
use uuid::Uuid;

pub async fn post(
    Json(login): Json<Credentials>,
    Extension(pool): Extension<PgPool>,
    Extension(store): Extension<RedisSessionStore>,
) -> impl IntoResponse {
    let user = query!(
        r#"
			SELECT id, password_hash
			FROM todo_user
			WHERE email = $1
        "#,
        login.email,
    )
    .fetch_one(&pool)
    .await
    .ok()
    .map(|user| User {
        id: user.id,
        password_hash: Secret::new(user.password_hash),
    });

    match validate_user(user, login.password).await {
        Ok(user_id) => match get_session_cookie(user_id, store).await {
            Ok(cookie) => {
                let headers = AppendHeaders([(SET_COOKIE, cookie)]);
                dbg!(headers);
                Ok(StatusCode::OK)
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn get_session_cookie(
    user_id: Uuid,
    store: RedisSessionStore,
) -> Result<Cookie<'static>, anyhow::Error> {
    let mut session = Session::new();
    session.expire_in(std::time::Duration::from_secs(4 * 60 * 60));
    session.insert(user_id.to_string().as_str(), user_id)?;

    let cookie_value = store
        .store_session(session)
        .await?
        .ok_or(anyhow::anyhow!("No cookie value found"))?;

    let cookie = Cookie::build("id", cookie_value)
        .http_only(true)
        .max_age(Duration::hours(4))
        .finish();
    let mut jar = CookieJar::new();
    let key =
        Key::from("super-long-and-secret-random-key-needed-to-verify-message-integrity".as_bytes());
    jar.private_mut(&key).add(cookie);
    let cookie = jar
        .delta()
        .next()
        .ok_or(anyhow::anyhow!("No cookie found"))?
        .clone();

    Ok(cookie)
}
