use secrecy::Secret;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub password_hash: Secret<String>,
}
