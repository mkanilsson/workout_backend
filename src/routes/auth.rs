use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{
    ctx::Ctx, dtos::auth::{CreateUserPayload, LoginPayload, LoginResponse, UserResponse}, error::{AuthError, Error, Result}, helpers::security::{hash_password, verify_password}, models::user::User, response::Response, ApiState
};
use crate::middlewares::auth::require_auth;

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/auth/refresh", get(refresh_token))
        .route("/api/auth/logout", delete(logout))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .with_state(state)
}

async fn register(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<Response<UserResponse>>)> {
    if let Some(_) = User::find_by_email(&state.db, &payload.email).await? {
        return Err(Error::AuthError(AuthError::EmailAlreadyInUse(
            payload.email,
        )));
    }

    let hashed_password = hash_password(&payload.password)?;

    let user = User::create(&state.db, payload.email, hashed_password).await?;

    Ok((StatusCode::CREATED, Json(Response::success(user.into()))))
}

async fn login(
    State(state): State<ApiState>,
    Json(payload): Json<LoginPayload>,
) -> Result<(StatusCode, Json<Response<LoginResponse>>)> {
    let user = User::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or(Error::NotFound("User".to_string()))?;

    if verify_password(&payload.password, &user.password)? {
        Ok((
            StatusCode::CREATED,
            Json(Response::success(LoginResponse {
                token: user.create_token(&state.db).await?.value,
                user: user.into(),
            })),
        ))
    } else {
        Err(Error::AuthError(AuthError::LoginFailed))
    }
}

async fn refresh_token(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<LoginResponse>>)> {
    let user = ctx.user().clone();
    let token = user.create_token(&state.db).await?.value;
    ctx.token().delete(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(Response::success(LoginResponse {
            token,
            user: user.into(),
        })),
    ))
}

async fn logout(State(state): State<ApiState>, ctx: Ctx) -> Result<Json<Value>> {
    let user = ctx.user().clone();

    for t in user.tokens(&state.db).await? {
        t.delete(&state.db).await?;
    }

    Ok(Json(json!({ "status": "Success", "message": "logged out" })))
}
