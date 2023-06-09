use async_std::task;
use mongodb::Database;
use crate::auth::data::entities::sign_up::sign_up_cloud_data::SignUpCloudData;
use crate::auth::domain::repository::auth_repository::AuthRepository;

pub struct AuthRepositoryImpl {
    database: Database
}

impl AuthRepositoryImpl {
    pub fn new(database: Database) -> Self {
        AuthRepositoryImpl {
            database
        }
    }
}

impl AuthRepository for AuthRepositoryImpl {
    fn sign_up(&self, sign_up_cloud_data: &SignUpCloudData) {
        let collection = self.database.collection::<SignUpCloudData>("users");
            task::block_on(async {
                collection.insert_one(sign_up_cloud_data, None).await.ok();
            }
        );
    }
}