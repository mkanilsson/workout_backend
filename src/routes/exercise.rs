use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::post,
    Json, Router,
};

use crate::{
    ctx::Ctx, dtos::exercise::CreateExercisePayload, error::Result, models::exercise::Exercise, ApiState
};
use crate::middlewares::auth::require_auth;

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/exercises", post(create_exercise))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .with_state(state)
}

async fn create_exercise(
    State(state): State<ApiState>,
    ctx: Ctx,
    Json(payload): Json<CreateExercisePayload>,
) -> Result<(StatusCode, Json<Exercise>)> {
    let user = ctx.user().clone();

    let exercice = Exercise::create(&state.db, user.id, payload.name, payload.exercise_type).await?;

    Ok((
        StatusCode::CREATED,
        Json(exercice),
    ))
}
