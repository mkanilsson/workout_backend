use axum::routing::get;
use axum::{extract::State, http::StatusCode, Json, Router};

use crate::dtos::target::TargetResponse;
use crate::models::target::Target;
use crate::response::Response;
use crate::{error::Result, ApiState};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/targets", get(get_targets))
        .with_state(state)
}

async fn get_targets(
    State(state): State<ApiState>,
) -> Result<(StatusCode, Json<Response<Vec<TargetResponse>>>)> {
    let targets = Target::all(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(Response::success(targets.iter().map(|t| t.into()).collect())),
    ))
}
