use jsonwebtoken::{Algorithm, decode, DecodingKey, TokenData, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenGenerator {
    secret: String,
}

impl TokenGenerator {
    pub fn new(secret: &str) -> Self {
        TokenGenerator {
            secret: secret.to_owned(),
        }
    }

    pub fn generate_token(&self, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let claims = jsonwebtoken::Claims {
            sub: user_id.to_owned(),
            iat: current_time as usize,
            exp: (current_time + 3600) as usize, // Token expiration time (1 hour)
            ..Default::default()
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            self.secret.as_bytes(),
        )
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenData<jsonwebtoken::Header, jsonwebtoken::Claims>, jsonwebtoken::errors::Error> {
        decode::<jsonwebtoken::Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
    }
}
