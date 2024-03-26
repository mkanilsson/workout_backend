use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

use super::{exercise::Exercise, exercise_workout::ExerciseWorkout};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum WorkoutStatus {
    Ongoing,
    Done
}

impl From<String> for WorkoutStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ongoing" => Self::Ongoing,
            "done" => Self::Done,
            _ => panic!("Unknown WorkoutStatus: {}", value),
        }
    }
}

impl ToString for WorkoutStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Ongoing => "ongoing",
            Self::Done => "done",
        }
        .to_string()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Workout {
    pub id: String,
    pub user_id: String,
    pub status: WorkoutStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Workout {
    pub async fn create(
        db: &Pool<MySql>,
        user_id: String,
    ) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO workout(id, user_id) VALUE (?, ?)",
            id,
            user_id,
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
            sqlx::query_as!(Workout, "SELECT * FROM workout WHERE id = ? LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_all_done_by_user_id(db: &Pool<MySql>, user_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Workout, "SELECT * FROM workout WHERE user_id = ? AND status = 'done'", user_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?
        )
    }

    pub async fn find_current_by_user_id(db: &Pool<MySql>, user_id: String) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Workout, "SELECT * FROM workout WHERE user_id = ? and status = 'ongoing'", user_id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn finish(&mut self, db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "UPDATE workout SET status = 'done' WHERE id = ?",
            self.id,
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        self.status = WorkoutStatus::Done;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub async fn exercise_workouts(&self, db: &Pool<MySql>) -> Result<Vec<ExerciseWorkout>> {
        Ok(sqlx::query_as!(ExerciseWorkout, "SELECT * FROM exercise_workout WHERE workout_id = ?", self.id)
            .fetch_all(db)
            .await
            .map_err(error::from_sqlx_error)?)
    }

    pub async fn find_all_where_exercised_is_used(db: &Pool<MySql>, exercise_id: String) -> Result<Vec<Workout>> {
        Ok(
            sqlx::query_as!(Workout, "SELECT * FROM workout WHERE id IN (SELECT workout_id FROM exercise_workout WHERE exercise_id = ?) ORDER BY created_at ASC", exercise_id)
            .fetch_all(db)
            .await
            .map_err(error::from_sqlx_error)?
        )
    }
}
