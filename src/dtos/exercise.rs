use crate::models::exercise::ExerciseType;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateExercisePayload {
    pub name: String,
    pub exercise_type: ExerciseType,
}
