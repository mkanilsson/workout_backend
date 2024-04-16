use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ExerciseTarget {
    pub id: String,
    pub exercise_id: String,
    pub target_id: String,
}

impl ExerciseTarget {
    pub async fn create(db: &Pool<MySql>, exercise_id: String, target_id: String) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO exercise_target (id, exercise_id, target_id) VALUES (?, ?, ?)",
            id,
            exercise_id,
            target_id,
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Self::find_by_id(db, id)
            .await?
            .ok_or(Error::WTF("Inserted ID doesn't exist".into()))
    }

    pub async fn find_by_id(db: &Pool<MySql>, id: String) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            ExerciseTarget,
            "SELECT * FROM exercise_target WHERE id = ?",
            id
        )
        .fetch_optional(db)
        .await
        .map_err(error::from_sqlx_error)?)
    }

    pub async fn delete_by_exercise_id(db: &Pool<MySql>, exercise_id: String) -> Result<()> {
        sqlx::query!(
            "DELETE FROM exercise_target WHERE exercise_id = ?",
            exercise_id
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Ok(())
    }
}
