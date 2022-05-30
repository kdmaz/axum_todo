use axum::{
    routing::{get, post},
    Router,
};
mod health_check;
mod login;
mod logout;
mod register;

pub fn get_router() -> Router {
    let app = Router::new()
        .route("/health_check", get(health_check::get))
        .route("/register", post(register::post))
        .route("/login", post(login::post))
        .route("/logout", post(logout::post));

    Router::new().nest("/api", app)
}
