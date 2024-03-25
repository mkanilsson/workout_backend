#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateExerciseWorkoutPayload {
    pub exercise_id: String,
}
