use crate::auth::data::entities::sign_up_data::SignUpData;

pub trait AuthRepository {
    fn sign_up(&self, sign_up_data: &SignUpData);
}