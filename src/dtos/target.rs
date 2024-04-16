use crate::models::target::Target;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TargetResponse {
    pub id: String,
    pub name: String,
}

impl From<&Target> for TargetResponse {
    fn from(value: &Target) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.to_string(),
        }
    }
}
