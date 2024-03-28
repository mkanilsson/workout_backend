use chrono::Utc;

use crate::models::{exercise::ExerciseType, set::Set, workout::WorkoutStatus};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DetailedWorkout {
    pub id: String,
    pub status: WorkoutStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub exercises: Vec<DetailedExercise>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DetailedExercise {
    pub id: String,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub exercise_workout_id: String,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub sets: Vec<Set>,
}
