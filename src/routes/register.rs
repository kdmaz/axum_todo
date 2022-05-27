use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{query, PgPool};
use todo::auth::compute_password_hash;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Registration {
    email: String,
    password: Secret<String>,
}

pub async fn post(
    Json(registration): Json<Registration>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4();
    let password_hash = match compute_password_hash(registration.password) {
        Ok(password_hash) => password_hash,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let rows = query!(
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
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            if let Some(code) = err.as_database_error().and_then(|err| err.code()) {
                // code for unique violation
                if code == "23505" {
                    // return created response to prevent account enumeration
                    return StatusCode::CREATED;
                }
            }

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
