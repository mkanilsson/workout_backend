use chrono::Utc;

use crate::models::{
    exercise::{Exercise, ExerciseType},
    set::Set,
    target::Target,
};

use super::target::TargetResponse;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateExercisePayload {
    pub name: String,
    pub exercise_type: ExerciseType,
    pub targets: Vec<String>, // ids
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ExerciseResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub targets: Vec<TargetResponse>,
}

impl ExerciseResponse {
    pub fn from_exercise_and_targets(exercise: Exercise, targets: Vec<Target>) -> Self {
        Self {
            id: exercise.id.clone(),
            user_id: exercise.user_id.clone(),
            name: exercise.name.clone(),
            exercise_type: exercise.exercise_type,
            created_at: exercise.created_at,
            updated_at: exercise.updated_at,

            targets: targets.iter().map(|t| t.into()).collect(),
        }
    }
}
