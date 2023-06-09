use crate::auth::data::entities::hashing::salted_hash::SaltedHash;

pub trait HashingService {
    fn generate_salt(&self, salt_length: usize) -> String;
    fn generate_salted_hash(&self, value: &str, salt: &str) -> SaltedHash;
    fn verify(&self, value: &str, salted_hash: &SaltedHash) -> bool;
}