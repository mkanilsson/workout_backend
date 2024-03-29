use chrono::Utc;

use crate::models::{exercise::ExerciseType, set::Set};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateExercisePayload {
    pub name: String,
    pub exercise_type: ExerciseType,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ExerciseHistoryPayload {
    pub workout_id: String,
    pub workout_date: chrono::DateTime<Utc>,
    pub exercise_type: ExerciseType,
    pub groups: Vec<ExerciseGroupHistoryPayload>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ExerciseGroupHistoryPayload {
    pub start_date: chrono::DateTime<Utc>,
    pub sets: Vec<Set>,
}
