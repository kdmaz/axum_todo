use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}

pub async fn validate_user(
    password_hash: Option<Secret<String>>,
    password: Secret<String>,
) -> Result<(), anyhow::Error> {
    // if no password_hash, still verify_password_hash to prevent user enumeration
    // through a timing attack
    let default_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
                gZiV/M1gPc22ElAH/Jh1Hw$\
                CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );
    let expected_password_hash = password_hash.unwrap_or(default_password_hash);

    tokio::task::spawn_blocking(move || verify_password_hash(expected_password_hash, password))
        .await?
}

pub fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), anyhow::Error> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())?;

    Ok(Argon2::default().verify_password(
        password_candidate.expose_secret().as_bytes(),
        &expected_password_hash,
    )?)
}
