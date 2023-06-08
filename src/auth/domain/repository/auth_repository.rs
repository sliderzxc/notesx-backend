use crate::auth::data::entities::sign_up_request_data::SignUpRequestData;

pub trait AuthRepository {
    fn sign_up(&self, sign_up_data: &SignUpRequestData);
}