use mongodb::Database;
use crate::auth::data::entities::sign_up_data::SignUpData;
use crate::auth::domain::repository::auth_repository::AuthRepository;

pub struct AuthRepositoryImpl {
    database: Database
}

impl AuthRepositoryImpl {
    pub fn new(database: Database) -> Self {
        AuthRepositoryImpl { database }
    }
}

impl AuthRepository for AuthRepositoryImpl {
    fn sign_up(&self, sign_up_data: &SignUpData) {
        self.database.collection::<SignUpData>("users").insert_one(sign_up_data, None);
    }
}