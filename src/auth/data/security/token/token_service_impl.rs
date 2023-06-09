use jsonwebtoken::{encode, EncodingKey, Header};
use crate::auth::data::entities::token::token_claim::TokenClaim;
use crate::auth::data::entities::token::token_config::TokenConfig;
use crate::auth::domain::security::token::token_service::TokenService;

impl TokenService for TokenServiceImpl {
    fn generate_token(claims: Vec<TokenClaim>) -> String {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref())
        )?;
        println!(token);

        token
    }
}