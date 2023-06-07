use std::env;
use dotenv::dotenv;
use mongodb::Client;

pub struct MongoDatabase;

impl MongoDatabase {
    pub async fn init() -> mongodb::Database {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        client.database("notes-api-database")
    }
}