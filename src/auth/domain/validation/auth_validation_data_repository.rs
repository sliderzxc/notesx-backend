use crate::auth::data::entities::sign_up::sign_up_request_data::SignUpRequestData;

pub trait AuthValidationDataRepository {
    fn validate_sign_up_data(&self, sign_up_data: &SignUpRequestData) -> bool;
}