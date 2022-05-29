use axum::{http::StatusCode, Extension, Json};
use secrecy::Secret;
use sqlx::{query, PgPool};
use todo::{
    auth::{validate_user, Credentials},
    user::User,
};

pub async fn post(
    Json(login): Json<Credentials>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
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
        Ok(user_id) => {
            dbg!(user_id);
            // TODO - session
            // store.create_session_with_user_id(user_id)

            StatusCode::OK
        }
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
