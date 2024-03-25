use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts, Request},
    http::{request::Parts, HeaderMap},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};

use crate::{
    ctx::Ctx,
    error::{AuthError::InvalidToken, Error, Result},
    models::token::Token,
    ApiState,
};

pub async fn require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {

    ctx?;

    Ok(next.run(req).await)
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    ApiState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let headers = parts.extract::<HeaderMap>().await.unwrap();
        let state = ApiState::from_ref(state);

        let Some(token) = headers.get("Authorization") else {
            return Err(Error::AuthError(InvalidToken));
        };

        let token = token.to_str().map_err(|_| Error::AuthError(InvalidToken))?;
        let token = Token::find_by_value(&state.db, token)
            .await?
            .ok_or(Error::AuthError(InvalidToken))?;

        let user = token
            .user(&state.db)
            .await?
            .ok_or(Error::WTF("Token exists but user doesn't".to_string()))?;

        Ok(Ctx::new(user, token))
    }
}
