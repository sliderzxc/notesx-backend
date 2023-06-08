use std::rc::Rc;
use crate::auth::data::entities::sign_up_data::SignUpData;
use crate::auth::domain::repository::auth_repository::AuthRepository;
use crate::auth::domain::validation::auth_validation_data_repository::AuthValidationDataRepository;

pub struct SignUpUseCase {
    auth_repository: Rc<dyn AuthRepository>,
    auth_validation_data_repository: Rc<dyn AuthValidationDataRepository>
}

impl SignUpUseCase {
    pub fn new(
        auth_repository: Rc<dyn AuthRepository>,
        auth_validation_data_repository: Rc<dyn AuthValidationDataRepository>
    ) -> Self {
        SignUpUseCase { auth_repository, auth_validation_data_repository }
    }

    pub fn execute(&self, sign_up_data: &SignUpData) -> bool {
        return if self.auth_validation_data_repository.validate_sign_up_data(sign_up_data) == false {
            false
        } else {
            self.auth_repository.sign_up(sign_up_data);
            true
        }
    }
}