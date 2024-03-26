use chrono::{DateTime, Utc};
use sqlx::{MySql, Pool};

use crate::error::{self, Error, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum SetType {
    Warmup,
    Normal,
}

impl From<String> for SetType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "warmup" => Self::Warmup,
            "normal" => Self::Normal,
            _ => panic!("Unknown SetType: {}", value),
        }
    }
}

impl ToString for SetType {
    fn to_string(&self) -> String {
        match self {
            Self::Warmup => "warmup",
            Self::Normal => "normal",
        }
        .to_string()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Set {
    pub id: String,
    pub user_id: String,
    pub exercise_workout_id: String,
    pub quality: f32,
    pub quantity: f32,
    pub note: Option<String>,
    pub set_type: SetType,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Set {
    pub async fn create(
        db: &Pool<MySql>,
        user_id: String,
        exercise_workout_id: String,
        quality: f32,
        quantity: f32,
        set_type: SetType,
    ) -> Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO sets(id, user_id, exercise_workout_id, quality, quantity, set_type) VALUE (?, ?, ?, ?, ?, ?)",
            id,
            user_id,
            exercise_workout_id,
            quality,
            quantity,
            set_type.to_string()
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
            sqlx::query_as!(Set, "SELECT * FROM sets WHERE id = ? LIMIT 1", id)
                .fetch_optional(db)
                .await
                .map_err(error::from_sqlx_error)?
        )

    }

    pub async fn find_all_by_exercise_workout_id(db: &Pool<MySql>, exercise_workout_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Set, "SELECT * FROM sets WHERE exercise_workout_id = ? ORDER BY set_type ASC, created_at ASC", exercise_workout_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?
        )

    }

    pub async fn find_all_by_user_id(db: &Pool<MySql>, user_id: String) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Set, "SELECT * FROM sets WHERE user_id = ? ORDER BY set_type ASC, created_at ASC", user_id)
                .fetch_all(db)
                .await
                .map_err(error::from_sqlx_error)?
        )
    }

    pub async fn save(&mut self, db: &Pool<MySql>) -> Result<()> {
        sqlx::query!(
            "UPDATE sets SET quality = ?, quantity = ?, set_type = ? WHERE id = ?",
            self.quality,
            self.quantity,
            self.set_type.to_string(),
            self.id
        )
        .execute(db)
        .await
        .map_err(error::from_sqlx_error)?;

        self.updated_at = Utc::now();

        Ok(())
    }
}
