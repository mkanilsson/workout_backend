use axum::extract::Path;
use axum::routing::{delete, get, put};
use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};

use crate::dtos::exercise_workout::CreateExerciseWorkoutPayload;
use crate::dtos::workout::{DetailedExercise, DetailedWorkout};
use crate::error::{AuthError, Error};
use crate::middlewares::auth::require_auth;
use crate::models::exercise_workout::ExerciseWorkout;
use crate::response::Response;
use crate::{ctx::Ctx, error::Result, models::workout::Workout, ApiState};

pub fn router(state: ApiState) -> Router {
    // TODO: change `/current` to `/:id` and add special case for current
    Router::new()
        .route("/api/workouts", post(create_workout))
        .route("/api/workouts", get(get_done_workouts)) // Old
        .route("/api/workouts/current", get(get_current_workout))
        .route("/api/workouts/current", put(finish_current_workout))
        .route(
            "/api/workouts/current/exercises",
            post(add_exercise_to_current_workout),
        )
        .route(
            "/api/workouts/current/exercises/:exercise_workout_id",
            delete(delete_exercise_to_current_workout),
        )
        .route("/api/workouts/:id", put(delete_workout))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .with_state(state)
}

async fn create_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<Workout>>)> {
    let user = ctx.user().clone();

    let workout = Workout::create(&state.db, user.id).await?;

    Ok((StatusCode::CREATED, Json(Response::success(workout))))
}

async fn get_done_workouts(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<Vec<Workout>>>)> {
    let workouts = ctx.user().workouts(&state.db).await?;

    Ok((StatusCode::OK, Json(Response::success(workouts))))
}

async fn get_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<DetailedWorkout>>)> {
    let workout = ctx.user().current_workout(&state.db).await?;

    let Some(workout) = workout else {
        return Err(Error::NotFound(format!(
            "Current workout for user {}",
            ctx.user().id
        )))
    };

    let exercise_workouts = workout.exercise_workouts(&state.db).await?;
    let mut detailed_exercises = vec![];

    for ew in exercise_workouts {
        let sets = ew.sets(&state.db).await?;
        let exercise = ew.exercise(&state.db).await?;

        let de = DetailedExercise {
            id: exercise.id.clone(),
            name: exercise.name.clone(),
            exercise_type: exercise.exercise_type,
            exercise_workout_id: ew.id.clone(),
            created_at: exercise.created_at,
            updated_at: exercise.updated_at,
            sets,
        };
        detailed_exercises.push(de);
    }

    Ok((
        StatusCode::OK,
        Json(Response::success(DetailedWorkout {
            id: workout.id.clone(),
            status: workout.status,
            created_at: workout.created_at,
            updated_at: workout.updated_at,
            exercises: detailed_exercises,
        })),
    ))
}

async fn finish_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<Response<Workout>>)> {
    let workout = ctx.user().current_workout(&state.db).await?;

    if let Some(mut workout) = workout {
        workout.finish(&state.db).await?;
        Ok((StatusCode::OK, Json(Response::success(workout))))
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
) -> Result<(StatusCode, Json<Response<ExerciseWorkout>>)> {
    let user = ctx.user();
    let workout = user.current_workout(&state.db).await?;

    if let Some(workout) = workout {
        let exercise_workout =
            ExerciseWorkout::create(&state.db, user.id.clone(), payload.exercise_id, workout.id)
                .await?;
        Ok((StatusCode::CREATED, Json(Response::success(exercise_workout))))
    } else {
        Err(Error::NotFound(format!(
            "Current workout for user {}",
            ctx.user().id
        )))
    }
}

async fn delete_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((id,)): Path<(String,)>,
) -> Result<(StatusCode, Json<Response<Workout>>)> {
    let user = ctx.user();
    let workout = Workout::find_by_id(&state.db, id.clone()).await?;

    let Some(mut workout) = workout else {
        return Err(Error::NotFound(format!(
            "Workout with id {}",
            id
        )));
    };

    if workout.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    workout.delete(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(Response::success(workout)),
    ))
}

async fn delete_exercise_to_current_workout(
    State(state): State<ApiState>,
    ctx: Ctx,
    Path((exercise_workout_id,)): Path<(String,)>,
) -> Result<(StatusCode, Json<Response<ExerciseWorkout>>)> {
    let user = ctx.user();
    let exercise_workout = ExerciseWorkout::find_by_id(&state.db, exercise_workout_id.clone()).await?;

    let Some(mut exercise_workout) = exercise_workout else {
        return Err(Error::NotFound(format!(
            "ExerciseWorkout with id {}",
            exercise_workout_id.clone()
        )));
    };

    if exercise_workout.user_id != user.id {
        return Err(Error::AuthError(AuthError::NotYourItem));
    }

    exercise_workout.delete(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(Response::success(exercise_workout)),
    ))
}
