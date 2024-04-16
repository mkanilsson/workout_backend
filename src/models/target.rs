use sqlx::{MySql, Pool};

use crate::error::{self, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Target {
    pub id: String,
    pub name: String,
    pub sort: i32,
}

impl Target {
    pub async fn all(db: &Pool<MySql>) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Target, "SELECT * FROM targets ORDER BY sort ASC")
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_by_id(db: &Pool<MySql>, id: String) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Target, "SELECT * FROM targets WHERE id = ?", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn all_by_exercise_id(db: &Pool<MySql>, exercise_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Target, "SELECT * FROM targets WHERE id IN (SELECT target_id FROM exercise_target WHERE exercise_id = ?) ORDER BY sort ASC", exercise_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }
}
