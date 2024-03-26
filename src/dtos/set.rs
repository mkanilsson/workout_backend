use crate::models::set::SetType;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateSetPayload {
    pub exercise_workout_id: String,
    pub quality: f32,
    pub quantity: f32,
    pub set_type: SetType,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UpdateSetPayload {
    pub quality: f32,
    pub quantity: f32,
    pub set_type: SetType,
}
