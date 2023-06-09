use std::rc::Rc;
use crate::auth::data::entities::sign_up::sign_up_cloud_data::SignUpCloudData;
use crate::auth::data::entities::sign_up::sign_up_request_data::SignUpRequestData;
use crate::auth::domain::repository::auth_repository::AuthRepository;
use crate::auth::domain::security::hashing::hashing_service::HashingService;
use crate::auth::domain::validation::auth_validation_data_repository::AuthValidationDataRepository;

pub struct SignUpUseCase {
    auth_repository: Rc<dyn AuthRepository>,
    auth_validation_data_repository: Rc<dyn AuthValidationDataRepository>,
    hashing_service: Rc<dyn HashingService>
}

impl SignUpUseCase {
    pub fn new(
        auth_repository: Rc<dyn AuthRepository>,
        auth_validation_data_repository: Rc<dyn AuthValidationDataRepository>,
        hashing_service: Rc<dyn HashingService>
    ) -> Self {
        SignUpUseCase {
            auth_repository, auth_validation_data_repository, hashing_service
        }
    }

    pub fn execute(&self, sign_up_data: &SignUpRequestData) -> bool {
        return if self.auth_validation_data_repository.validate_sign_up_data(sign_up_data) == false {
            false
        } else {
            let salt = self.hashing_service.generate_salt(sign_up_data.password.len());
            let salted_hash = self.hashing_service.generate_salted_hash(
                &sign_up_data.password, &salt
            );
            let sign_up_cloud_data = SignUpCloudData {
                id: None,
                email: sign_up_data.email.to_string(),
                password: salted_hash.hash,
                salt: salted_hash.salt,
            };
            self.auth_repository.sign_up(&sign_up_cloud_data);
            true
        }
    }
}