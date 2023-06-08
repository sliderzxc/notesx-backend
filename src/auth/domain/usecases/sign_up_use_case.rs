use std::rc::Rc;
use crate::auth::data::entities::sign_up_data::SignUpData;
use crate::auth::domain::repository::auth_repository::AuthRepository;

pub struct SignUpUseCase {
    pub(crate) auth_repository: Rc<dyn AuthRepository>,
}

impl SignUpUseCase {
    pub fn new(auth_repository: Rc<dyn AuthRepository>) -> Self {
        SignUpUseCase { auth_repository }
    }

    pub fn execute(&self, sign_up_data: &SignUpData) {
        self.auth_repository.sign_up(sign_up_data)
    }
}