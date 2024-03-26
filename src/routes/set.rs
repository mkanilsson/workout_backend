use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};

use crate::dtos::set::{CreateSetPayload, UpdateSetPayload};
use crate::error::{AuthError, Error};
use crate::middlewares::auth::require_auth;
use crate::models::set::Set;
use crate::{ctx::Ctx, error::Result, ApiState};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/sets", post(create_set))
        .route("/api/sets/:id", put(update_set))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .with_state(state)
}

async fn create_set(
    State(state): State<ApiState>,
    ctx: Ctx,
    Json(payload): Json<CreateSetPayload>,
) -> Result<(StatusCode, Json<Set>)> {
    let user = ctx.user();

    let set = Set::create(&state.db, user.id.clone(), payload.exercise_workout_id, payload.quality, payload.quantity, payload.set_type).await?;

    Ok((
        StatusCode::CREATED,
        Json(set),
    ))
}

async fn update_set(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((id,)): Path<(String,)>,
    Json(payload): Json<UpdateSetPayload>,
) -> Result<(StatusCode, Json<Set>)> {
    let user = ctx.user();

    let set = Set::find_by_id(&state.db, id.clone()).await?;

    let Some(mut set) = set else {
        return Err(Error::NotFound(format!(
            "Set with id {}",
            id
        )));
    };

    if set.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    set.quality = payload.quality;
    set.quantity = payload.quantity;
    set.set_type = payload.set_type;

    set.save(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(set),
    ))
}
