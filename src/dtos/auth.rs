use chrono::Utc;

use crate::models::user::User;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateUserPayload {
    pub email: String,
    pub password: String,
}

pub type LoginPayload = CreateUserPayload;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UserResponse {
    id: String,
    email: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
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
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
