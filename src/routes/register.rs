use axum::{http::StatusCode, Extension, Json};
use secrecy::ExposeSecret;
use sqlx::{query, PgPool};
use todo::auth::{compute_password_hash, Credentials};
use uuid::Uuid;

pub async fn post(
    Json(registration): Json<Credentials>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    let user_id = Uuid::new_v4();
    let password_hash = match compute_password_hash(registration.password) {
        Ok(password_hash) => password_hash,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let _ = query!(
        r#"
            INSERT INTO todo_user
                (id, email, password_hash)
            VALUES
                ($1, $2, $3);
        "#,
        user_id,
        registration.email,
        password_hash.expose_secret().to_string()
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        if let Some(code) = e.as_database_error().and_then(|err| err.code()) {
            // code for unique violation
            if code == "23505" {
                // return created response to prevent account enumeration
                return StatusCode::CREATED;
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR
    });

    StatusCode::CREATED
}
