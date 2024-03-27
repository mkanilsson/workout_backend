#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Response<T> {
    status: ResponseStatus,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: "Success".into(),
            data: Some(data),
        }
    }

    pub fn failure(message: &str) -> Self {
        Self {
            status: ResponseStatus::Failure,
            message: message.into(),
            data: None,
        }
    }
}
