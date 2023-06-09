use crate::auth::data::entities::sign_up::sign_up_cloud_data::SignUpCloudData;

pub trait AuthRepository {
    fn sign_up(&self, sign_up_cloud_data: &SignUpCloudData);
}