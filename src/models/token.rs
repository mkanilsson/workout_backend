use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::{
    error::{self, Error, Result},
    helpers::security::generate_token,
};

use super::user::User;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub value: String,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Token {
    pub async fn create(db: &Pool<MySql>, user_id: String) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO tokens(id, user_id, value) VALUE (?, ?, ?)",
            id,
            user_id,
            generate_token()
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Self::find_by_id(db, id)
            .await?
            .ok_or(Error::WTF("Inserted ID doesn't exist".into()))
    }

    pub async fn find_by_id(db: &Pool<MySql>, id: String) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Token, "SELECT * FROM tokens WHERE id = ? AND created_at > (NOW() - INTERVAL 1 WEEK) LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_by_value(db: &Pool<MySql>, value: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Token, "SELECT * FROM tokens WHERE value = ? AND created_at > (NOW() - INTERVAL 1 WEEK) LIMIT 1", value)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_all_by_user_id(db: &Pool<MySql>, user_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Token, "SELECT * FROM tokens WHERE user_id = ? AND created_at > (NOW() - INTERVAL 1 WEEK)", user_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn delete(&self, db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "DELETE FROM tokens WHERE id = ?",
            self.id
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Ok(())
    }

    pub async fn delete_expired(db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "DELETE FROM tokens WHERE created_at > (NOW() - INTERVAL 1 WEEK)",
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Ok(())
    }

    pub async fn user(&self, db: &Pool<MySql>) -> Result<Option<User>> {
        User::find_by_id(db, self.user_id.clone()).await
    }
}
