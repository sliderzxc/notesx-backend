use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use crate::auth::data::entities::token::token_claim::TokenClaim;
use crate::auth::data::entities::token::token_config::TokenConfig;
use crate::auth::domain::security::token::token_service::TokenService;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct CustomClaims {
    iss: String,
    sub: String,
    exp: usize,
    aud: Vec<String>,
    // Add any additional claim fields you need
    #[serde(flatten)]
    additional_claims: HashMap<String, String>,
}

impl Default for CustomClaims {
    fn default() -> Self {
        CustomClaims {
            iss: String::default(),
            sub: String::default(),
            exp: 0,
            aud: Vec::default(),
            additional_claims: HashMap::default(),
        }
    }
}

struct TokenServiceImpl {}

impl TokenService for TokenServiceImpl {
    fn generate(
        &self,
        config: &TokenConfig,
        claims: &[TokenClaim]
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(config.expires_in);

        let mut custom_claims = CustomClaims {
            iss: config.issuer.clone(),
            sub: String::default(),
            exp: expiration.timestamp() as usize,
            aud: vec![config.audience.clone()],
            additional_claims: HashMap::default(),
        };

        for claim in claims {
            custom_claims.additional_claims.insert(claim.name.clone(), claim.value.clone());
        }

        let token = encode(
            &Header::new(Algorithm::HS256),
            &custom_claims,
            &EncodingKey::from_secret(config.secret.as_bytes()),
        )?;

        Ok(token)
    }
}