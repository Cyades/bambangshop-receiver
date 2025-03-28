use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct SubscriberRequest {
    pub url: String,
    pub name: String,
}