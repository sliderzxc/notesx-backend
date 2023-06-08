use crate::auth::data::entities::token::token_claim::TokenClaim;
use crate::auth::data::entities::token::token_config::TokenConfig;

pub trait TokenService {

    fn generate_token(config: TokenConfig, claims: Vec<TokenClaim>) -> String;
}