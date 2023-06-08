use crate::auth::data::entities::token::token_claim::TokenClaim;
use crate::auth::data::entities::token::token_config::TokenConfig;
use crate::auth::domain::security::token::token_service::TokenService;

impl TokenService for TokenServiceImpl {
    fn generate_token(config: TokenConfig, claims: Vec<TokenClaim>) -> String {
        todo!()
    }
}