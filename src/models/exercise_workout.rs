use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

use super::{
    exercise::Exercise,
    set::{Set, SetType},
};

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
        Ok(sqlx::query_as!(
            ExerciseWorkout,
            "SELECT * FROM exercise_workout WHERE id = ? LIMIT 1",
            id
        )
        .fetch_optional(db)
        .await
        .map_err(error::from_sqlx_error)?)
    }

    pub async fn find_all_by_exercise_and_workout_id(db: &Pool<MySql>, exercise_id: String, workout_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(ExerciseWorkout, "SELECT * FROM exercise_workout WHERE exercise_id = ? AND workout_id = ?", exercise_id, workout_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?
        )
    }

    pub async fn add_set(
        &self,
        db: &Pool<MySql>,
        quality: f32,
        quantity: f32,
        set_type: SetType,
    ) -> Result<Set> {
        Set::create(
            db,
            self.user_id.clone(),
            self.id.clone(),
            quality,
            quantity,
            set_type,
        )
        .await
    }

    pub async fn exercise(&self, db: &Pool<MySql>) -> Result<Exercise> {
        Ok(Exercise::find_by_id(db, self.exercise_id.clone())
            .await?
            .ok_or(Error::WTF(
                "ExerciseWorkout exists but referenced exercise doesn't".into(),
            ))?)
    }

    pub async fn sets(&self, db: &Pool<MySql>) -> Result<Vec<Set>> {
        Ok(sqlx::query_as!(
            Set,
            "SELECT * FROM sets WHERE exercise_workout_id = ? ORDER BY set_type ASC, created_at ASC",
            self.id
        )
        .fetch_all(db)
        .await
        .map_err(error::from_sqlx_error)?)
    }
}
