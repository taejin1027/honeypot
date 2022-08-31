use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct User {
    /// Document Id
    pub _id: String,
    /// customer name
    pub id: String,
    /// createdAt
    pub pw: String,
}