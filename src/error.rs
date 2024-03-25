use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum AuthError {
    LoginFailed,
    EmailAlreadyInUse(String),
    InvalidToken,
}

#[derive(Debug)]
pub enum Error {
    Sql(sqlx::error::Error),
    AuthError(AuthError),
    NotFound(String),
    Other(String),
    WTF(String),
}

pub fn from_sqlx_error(err: sqlx::error::Error) -> Error {
    Error::Sql(err)
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthError(AuthError::EmailAlreadyInUse(_)) => StatusCode::CONFLICT,
            Self::AuthError(AuthError::LoginFailed) => StatusCode::UNAUTHORIZED,
            Self::AuthError(AuthError::InvalidToken) => StatusCode::UNAUTHORIZED,
            Self::Sql(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WTF(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::AuthError(AuthError::LoginFailed) => "Unauthorized",
            Self::AuthError(AuthError::InvalidToken) => "Missing token",
            Self::AuthError(AuthError::EmailAlreadyInUse(_)) => "Email already in use",
            Self::NotFound(_) => "Not Found",
            Self::Sql(_) | Self::Other(_) | Self::WTF(_) => "Internal server error",
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("Returning error: {self:?}");

        (
            self.status_code(),
            Json(json!({ "message": self.message() })),
        )
            .into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
