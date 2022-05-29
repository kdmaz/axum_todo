use cookie::{Cookie, CookieJar, Key};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

const COOKIE_NAME: &str = "id";

pub struct Store;

pub struct RedisSessionStore {
    // "super-long-and-secret-random-key-needed-to-verify-message-integrity";
    secret_key: Key,
    connection_string: String,
}

impl RedisSessionStore {
    pub fn new(secret_key: Key, connection_string: String) -> Self {
        RedisSessionStore {
            secret_key,
            connection_string,
        }
    }

    pub fn create_session_with_user_id(&self /* req, user_id */) {
        let session_key = generate_session_key();

        // add session_key -> user_id to redis
        // add user_id -> Set(session_keys) to redis

        let cookie = Cookie::new(COOKIE_NAME, session_key);
        let mut jar = CookieJar::new();
        jar.private_mut(&self.secret_key).add(cookie);
        let _cookie = jar.delta().next().unwrap();

        // add cookie to req
    }

    pub fn retrieve_user_id_from_session(&self /* req */) /* -> Uuid (user_id) */
    {
        // let mut jar = CookieJar::new();
        // req.cookies.ok() -> find cookie with name == COOKIE_NAME
        // jar.add_original(cookie.clone());
        // let session_key = jar.private_mut(&key).get(COOKIE_NAME);

        // get user_id from redis with session_key
    }

    pub fn clear_session(&self /* req, session_key */) {
        // get user_id from redis with session_key
        // remove session_key -> user_id from redis
        // remove session_key from user_id -> Set(session_keys)
    }
}

/// This session key generation routine follows [OWASP's recommendations](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#session-id-entropy).
fn generate_session_key() -> String {
    let value = std::iter::repeat(())
        .map(|()| OsRng.sample(Alphanumeric))
        .take(64)
        .collect::<Vec<_>>();
    // These unwraps will never panic because pre-conditions are always verified
    // (i.e. length and character set)
    String::from_utf8(value).unwrap().try_into().unwrap()
}
