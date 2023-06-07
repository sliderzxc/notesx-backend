extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::sync::Client;
use mongodb::sync::Database;

pub struct MongoRepo;

impl MongoRepo {
    pub fn init() -> Database {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        client.database("notes-api-database")
    }
}