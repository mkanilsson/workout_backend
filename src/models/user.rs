use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

use super::{exercise::Exercise, token::Token, workout::Workout};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl User {
    pub async fn create(db: &Pool<MySql>, email: String, hashed_password: String) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO users(id, email, password) VALUE (?, ?, ?)",
            id,
            email,
            hashed_password
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Self::find_by_id(db, id)
            .await?
            .ok_or(Error::WTF("Inserted ID doesn't exist".into()))
    }

    pub async fn find_by_email(db: &Pool<MySql>, email: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(User, "SELECT * FROM users WHERE email = ? LIMIT 1", email)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_by_id(db: &Pool<MySql>, id: String) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(User, "SELECT * FROM users WHERE id = ? LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn create_token(&self, db: &Pool<MySql>) -> Result<Token> {
        Token::create(db, self.id.clone()).await
    }

    pub async fn tokens(&self, db: &Pool<MySql>) -> Result<Vec<Token>> {
        Token::find_all_by_user_id(db, self.id.clone()).await
    }

    pub async fn exercises(&self, db: &Pool<MySql>) -> Result<Vec<Exercise>> {
        Exercise::find_all_by_user_id(db, self.id.clone()).await
    }

    pub async fn workouts(&self, db: &Pool<MySql>) -> Result<Vec<Workout>> {
        Workout::find_all_done_by_user_id(db, self.id.clone()).await
    }

    pub async fn current_workout(&self, db: &Pool<MySql>) -> Result<Option<Workout>> {
        Workout::find_current_by_user_id(db, self.id.clone()).await
    }
}
