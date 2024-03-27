use axum::extract::Path;
use axum::routing::{delete, get, put};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::post,
    Json, Router,
};

use crate::dtos::exercise::{ExerciseGroupHistoryPayload, ExerciseHistoryPayload};
use crate::error::{AuthError, Error};
use crate::models::exercise_workout::ExerciseWorkout;
use crate::models::set::Set;
use crate::models::workout::Workout;
use crate::response::Response;
use crate::{
    ctx::Ctx, dtos::exercise::CreateExercisePayload, error::Result, models::exercise::Exercise, ApiState
};
use crate::middlewares::auth::require_auth;

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/exercises", post(create_exercise))
        .route("/api/exercises", get(get_exercises))
        .route("/api/exercises/:id", put(update_exercise))
        .route("/api/exercises/:id/history", get(get_exercise_history))
        .route("/api/exercises/:id", delete(delete_exercise))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .with_state(state)
}

async fn create_exercise(
    State(state): State<ApiState>,
    ctx: Ctx,
    Json(payload): Json<CreateExercisePayload>,
) -> Result<(StatusCode, Json<Response<Exercise>>)> {
    let user = ctx.user().clone();

    let exercice = Exercise::create(&state.db, user.id, payload.name, payload.exercise_type).await?;

    Ok((
        StatusCode::CREATED,
        Json(Response::success(exercice)),
    ))
}

async fn get_exercises(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<Vec<Exercise>>>)> {
    let exercises = ctx.user().exercises(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(Response::success(exercises)),
    ))
}

async fn update_exercise(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((id,)): Path<(String,)>,
    Json(payload): Json<CreateExercisePayload>,
) -> Result<(StatusCode, Json<Response<Exercise>>)> {
    let user = ctx.user();
    let exercise = Exercise::find_by_id(&state.db, id.clone()).await?;

    let Some(mut exercise) = exercise else {
        return Err(Error::NotFound(format!(
            "Exercise with id {}",
            id
        )));
    };

    if exercise.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    exercise.name = payload.name;
    exercise.exercise_type = payload.exercise_type;

    exercise.save(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(Response::success(exercise)),
    ))
}

async fn get_exercise_history(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((id,)): Path<(String,)>,
) -> Result<(StatusCode, Json<Response<Vec<ExerciseHistoryPayload>>>)> {
    let user = ctx.user();
    // NOTE: This is pretty pointless but i like verifying the user before
    //       fetching all exercise_workouts because if the the list
    //       is empty, it would give back an empty list instead of
    //       an auth error.
    let exercise = Exercise::find_by_id(&state.db, id.clone()).await?;

    let Some(exercise) = exercise else {
        return Err(Error::NotFound(format!(
            "Exercise with id {}",
            id
        )));
    };

    if exercise.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    let workouts = Workout::find_all_where_exercised_is_used(&state.db, exercise.id.clone()).await?;

    let mut all = vec![];

    for w in workouts {
        let exercise_workouts = ExerciseWorkout::find_all_by_exercise_and_workout_id(&state.db, exercise.id.clone(), w.id.clone()).await?;

        let mut groups = vec![];

        for ew in exercise_workouts {
            groups.push(ExerciseGroupHistoryPayload {
                start_date: ew.created_at,
                sets: Set::find_all_by_exercise_workout_id(&state.db, ew.id.clone()).await?
            });
        }

        all.push(ExerciseHistoryPayload {
            workout_id: w.id.clone(),
            workout_date: w.created_at,
            groups,
        })
    }

    Ok((
        StatusCode::OK,
        Json(Response::success(all)),
    ))
}

async fn delete_exercise(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((id,)): Path<(String,)>,
) -> Result<(StatusCode, Json<Response<Exercise>>)> {
    let user = ctx.user();
    let exercise = Exercise::find_by_id(&state.db, id.clone()).await?;

    let Some(mut exercise) = exercise else {
        return Err(Error::NotFound(format!(
            "Exercise with id {}",
            id
        )));
    };

    if exercise.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    exercise.delete(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(Response::success(exercise)),
    ))
}
