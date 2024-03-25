use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ExerciseWorkout {
    pub id: String,
    pub user_id: String,
    pub exercise_id: String,
    pub workout_id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl ExerciseWorkout {
    pub async fn create(
        db: &Pool<MySql>,
        user_id: String,
        exercise_id: String,
        workout_id: String,
    ) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO exercise_workout(id, user_id, exercise_id, workout_id) VALUE (?, ?, ?, ?)",
            id,
            user_id,
            exercise_id,
            workout_id,
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
            sqlx::query_as!(ExerciseWorkout, "SELECT * FROM exercise_workout WHERE id = ? LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?
        )
    }
}
