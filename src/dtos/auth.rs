use mongodb::bson::{serde_helpers::serialize_bson_datetime_as_rfc3339_string, DateTime};

use crate::models::user::User;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPayload {
    pub email: String,
    pub password: String,
}

pub type LoginPayload = CreateUserPayload;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    id: String,
    email: String,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    updated_at: DateTime,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
