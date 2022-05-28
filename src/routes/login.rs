use axum::{http::StatusCode, Extension, Json};
use secrecy::Secret;
use sqlx::{query, PgPool};
use todo::auth::{validate_user, Credentials};

pub async fn post(
    Json(login): Json<Credentials>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    let user = query!(
        r#"
			SELECT email, password_hash
			FROM todo_user
			WHERE email = $1
        "#,
        login.email,
    )
    .fetch_one(&pool)
    .await;

    match validate_user(
        match user {
            Ok(user) => Some(Secret::new(user.password_hash)),
            Err(_) => None,
        },
        login.password,
    )
    .await
    {
        Ok(_) => {
            // TODO - session
            StatusCode::OK
        }
        Err(_) => StatusCode::UNAUTHORIZED
    }
}
