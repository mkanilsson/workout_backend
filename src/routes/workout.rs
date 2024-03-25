use axum::routing::{get, put};
use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};

use crate::dtos::exercise_workout::CreateExerciseWorkoutPayload;
use crate::error::Error;
use crate::middlewares::auth::require_auth;
use crate::models::exercise_workout::ExerciseWorkout;
use crate::{ctx::Ctx, error::Result, models::workout::Workout, ApiState};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/workouts", post(create_workout))
        .route("/api/workouts", get(get_done_workouts)) // Old
        .route("/api/workouts/current", get(get_current_workout))
        .route("/api/workouts/current", put(finish_current_workout))
        .route(
            "/api/workouts/current/exercises",
            post(add_exercise_to_current_workout),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .with_state(state)
}

async fn create_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Workout>)> {
    let user = ctx.user().clone();

    let workout = Workout::create(&state.db, user.id).await?;

    Ok((StatusCode::CREATED, Json(workout)))
}

async fn get_done_workouts(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Vec<Workout>>)> {
    let workouts = ctx.user().workouts(&state.db).await?;

    Ok((StatusCode::OK, Json(workouts)))
}

async fn get_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Workout>)> {
    let workout = ctx.user().current_workout(&state.db).await?;

    if let Some(workout) = workout {
        Ok((StatusCode::OK, Json(workout)))
    } else {
        Err(Error::NotFound(format!(
            "Current workout for user {}",
            ctx.user().id
        )))
    }
}

async fn finish_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Workout>)> {
    let workout = ctx.user().current_workout(&state.db).await?;

    if let Some(mut workout) = workout {
        workout.finish(&state.db).await?;
        Ok((StatusCode::OK, Json(workout)))
    } else {
        Err(Error::NotFound(format!(
            "Current workout for user {}",
            ctx.user().id
        )))
    }
}

async fn add_exercise_to_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
    Json(payload): Json<CreateExerciseWorkoutPayload>,
) -> Result<(StatusCode, Json<ExerciseWorkout>)> {
    let user = ctx.user();
    let workout = user.current_workout(&state.db).await?;

    if let Some(workout) = workout {
        let exercise_workout =
            ExerciseWorkout::create(&state.db, user.id.clone(), payload.exercise_id, workout.id)
                .await?;
        Ok((StatusCode::CREATED, Json(exercise_workout)))
    } else {
        Err(Error::NotFound(format!(
            "Current workout for user {}",
            ctx.user().id
        )))
    }
}
