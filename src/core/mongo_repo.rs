use std::env;
use dotenv::dotenv;
use mongodb::sync::Client;


impl MongoRepo {
    pub fn init() -> Client {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        client
    }
}