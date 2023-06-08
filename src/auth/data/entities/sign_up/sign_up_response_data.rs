use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignUpResponseData {
    pub token: String
}