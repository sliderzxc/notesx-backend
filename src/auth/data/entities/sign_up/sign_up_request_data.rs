use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignUpRequestData {
    pub email: String,
    pub password: String
}