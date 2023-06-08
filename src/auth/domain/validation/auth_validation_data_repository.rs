use crate::auth::data::entities::sign_up_data::SignUpData;

pub trait AuthValidationDataRepository {
    fn validate_sign_up_data(&self, sign_up_data: &SignUpData) -> bool;
}