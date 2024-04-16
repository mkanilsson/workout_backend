use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub enum ExerciseType {
    #[default]
    Static,
    DistanceOverTime,
    WeightOverAmount,
}

impl From<String> for ExerciseType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "static" => Self::Static,
            "distance_over_time" => Self::DistanceOverTime,
            "weight_over_amount" => Self::WeightOverAmount,
            _ => panic!("Unknown ExerciseType: {}", value),
        }
    }
}

impl ToString for ExerciseType {
    fn to_string(&self) -> String {
        match self {
            Self::Static => "static",
            Self::DistanceOverTime => "distance_over_time",
            Self::WeightOverAmount => "weight_over_amount",
        }
        .to_string()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Exercise {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Exercise {
    pub async fn create(
        db: &Pool<MySql>,
        user_id: String,
        name: String,
        exercise_type: ExerciseType,
    ) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO exercises(id, user_id, name, exercise_type) VALUE (?, ?, ?, ?)",
            id,
            user_id,
            name,
            exercise_type.to_string()
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
            sqlx::query_as!(Exercise, "SELECT * FROM exercises WHERE id = ? LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?,
        )
    }

    pub async fn find_all_by_user_id(db: &Pool<MySql>, user_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Exercise, "SELECT * FROM exercises WHERE user_id = ? ORDER BY name", user_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?
        )
    }

    pub async fn save(&mut self, db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "UPDATE exercises SET name = ?, exercise_type = ? WHERE id = ?",
            self.name,
            self.exercise_type.to_string(),
            self.id
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        self.updated_at = Utc::now();

        Ok(())
    }

    pub async fn delete(&mut self, db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "DELETE FROM exercises WHERE id = ?",
            self.id
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        Ok(())
    }
}
