use regex::Regex;
use crate::auth::data::entities::sign_up::sign_up_request_data::SignUpRequestData;
use crate::auth::domain::validation::auth_validation_data_repository::AuthValidationDataRepository;

pub struct AuthValidationDataRepositoryImpl;

impl AuthValidationDataRepository for AuthValidationDataRepositoryImpl {
    fn validate_sign_up_data(&self, sign_up_data: &SignUpRequestData) -> bool {
        let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();

        if !email_regex.is_match(&sign_up_data.email) {
            return false;
        }

        if sign_up_data.password.len() < 6 {
            return false;
        }

        let mut has_letter = false;
        let mut has_number = false;

        for ch in sign_up_data.password.chars() {
            if ch.is_ascii_alphabetic() {
                has_letter = true;
            } else if ch.is_ascii_digit() {
                has_number = true;
            }
        }

        has_letter && has_number
    }
}