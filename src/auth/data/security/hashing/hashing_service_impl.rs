use rand::Rng;
use sha2::{Digest, Sha256};
use crate::auth::data::entities::hashing::salted_hash::SaltedHash;
use crate::auth::domain::security::hashing::hashing_service::HashingService;

pub struct SHA256HashingService {}

impl HashingService for SHA256HashingService {
    fn generate_salt(&self, salt_length: usize) -> String {
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = (0..salt_length)
            .map(|_| rng.gen::<u8>())
            .collect();
        hex::encode(salt)
    }

    fn generate_salted_hash(&self, value: &str, salt: &str) -> SaltedHash {
        let salted_value = format!("{}{}", salt, value);
        let hash = Sha256::digest(salted_value.as_bytes());
        SaltedHash {
            hash: hex::encode(hash),
            salt: salt.to_owned(),
        }
    }

    fn verify(&self, value: &str, salted_hash: &SaltedHash) -> bool {
        let salted_value = format!("{}{}", salted_hash.salt, value);
        let hash = Sha256::digest(salted_value.as_bytes());
        hex::encode(hash) == salted_hash.hash
    }
}
