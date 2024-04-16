// I Don't write test is I use rust test framework as a db seeder :)

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use sqlx::mysql::MySqlPoolOptions;

    use crate::models::{
        exercise::{Exercise, ExerciseType},
        exercise_target::ExerciseTarget,
        exercise_workout::ExerciseWorkout,
        set::{Set, SetType},
        target::Target,
        user::{self, User},
        workout::Workout,
    };

    #[tokio::test]
    async fn seed() {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:root@localhost/workout")
            .await
            .unwrap();

        let mut targets: HashMap<String, String> = HashMap::new();
        for target in Target::all(&pool)
            .await
            .expect("Failed to retrive all targets")
        {
            targets.insert(target.name, target.id);
        }

        // password = "password"
        let user = User::create(&pool, "example@example.com".into(), "$argon2id$v=19$m=19456,t=2,p=1$lbgGK0mN9O7tZCwgbxN2jg$7D/pOtOXhHJxewLZJL3pvLUN3rjSNdLPnhGZV/NFTis".into()).await.expect("Failed to create user");
        let bench_press = Exercise::create(
            &pool,
            user.id.clone(),
            "Bench press".into(),
            ExerciseType::WeightOverAmount,
        )
        .await
        .expect("Failed to create 'Bench press'");
        ExerciseTarget::create(&pool, bench_press.id.clone(), targets["Chest"].clone()).await.expect("Failed to mark bench press as chest exercise");
        ExerciseTarget::create(&pool, bench_press.id.clone(), targets["Triceps"].clone()).await.expect("Failed to mark bench press as triceps exercise");

        let running = Exercise::create(
            &pool,
            user.id.clone(),
            "Running".into(),
            ExerciseType::DistanceOverTime,
        )
        .await
        .expect("Failed to create 'Running'");
        ExerciseTarget::create(&pool, running.id.clone(), targets["Cardio"].clone()).await.expect("Failed to mark running as cardio exercise");

        let squats = Exercise::create(
            &pool,
            user.id.clone(),
            "Squats".into(),
            ExerciseType::WeightOverAmount,
        )
        .await
        .expect("Failed to create 'Squats'");
        ExerciseTarget::create(&pool, squats.id.clone(), targets["Thighs"].clone()).await.expect("Failed to mark squats as thighs exercise");

        let counter_rotation = Exercise::create(
            &pool,
            user.id.clone(),
            "Counter rotation".into(),
            ExerciseType::Static,
        )
        .await
        .expect("Failed to create 'Counter rotation'");
        ExerciseTarget::create(&pool, counter_rotation.id.clone(), targets["Core"].clone()).await.expect("Failed to mark counter rotation as core exercise");

        let mut workout1 = Workout::create(&pool, user.id.clone())
            .await
            .expect("Failed to create 'workout1");
        // Sleeping to make the datetime different
        tokio::time::sleep(Duration::from_secs(1)).await;
        let bench_press_workout1 = ExerciseWorkout::create(
            &pool,
            user.id.clone(),
            bench_press.id.clone(),
            workout1.id.clone(),
        )
        .await
        .expect("Failed to add 'bench press' to 'workout1'");
        tokio::time::sleep(Duration::from_secs(1)).await;

        bench_press_workout1
            .add_set(&pool, 20.0, 12.0, SetType::Warmup)
            .await
            .expect("Failed to add warmup1 to benchpress workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        bench_press_workout1
            .add_set(&pool, 30.0, 8.0, SetType::Warmup)
            .await
            .expect("Failed to add warmup2 to benchpress workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        bench_press_workout1
            .add_set(&pool, 40.0, 8.0, SetType::Normal)
            .await
            .expect("Failed to add normal1 to benchpress workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        bench_press_workout1
            .add_set(&pool, 42.5, 6.0, SetType::Normal)
            .await
            .expect("Failed to add normal2 to benchpress workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let squats_workout1 = ExerciseWorkout::create(
            &pool,
            user.id.clone(),
            squats.id.clone(),
            workout1.id.clone(),
        )
        .await
        .expect("Failed to add 'squats' to 'workout1'");
        tokio::time::sleep(Duration::from_secs(1)).await;
        squats_workout1
            .add_set(&pool, 20.0, 12.0, SetType::Warmup)
            .await
            .expect("Failed to add warmup1 to squats workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        squats_workout1
            .add_set(&pool, 30.0, 8.0, SetType::Normal)
            .await
            .expect("Failed to add normal1 to squats workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        squats_workout1
            .add_set(&pool, 40.0, 8.0, SetType::Normal)
            .await
            .expect("Failed to add normal2 to squats workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        squats_workout1
            .add_set(&pool, 50.0, 3.0, SetType::Normal)
            .await
            .expect("Failed to add normal3 to squats workout1");
        tokio::time::sleep(Duration::from_secs(1)).await;

        workout1
            .finish(&pool)
            .await
            .expect("Failed to finish workout1");

        let workout2 = Workout::create(&pool, user.id.clone())
            .await
            .expect("Failed to create 'workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let counter_rotation_workout2 = ExerciseWorkout::create(
            &pool,
            user.id.clone(),
            counter_rotation.id.clone(),
            workout2.id.clone(),
        )
        .await
        .expect("Failed to add 'counter rotation' to 'workout2'");
        tokio::time::sleep(Duration::from_secs(1)).await;

        counter_rotation_workout2
            .add_set(&pool, 23.0, 20.0, SetType::Normal)
            .await
            .expect("Failed to add warmup1 to counter rotation workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;
        counter_rotation_workout2
            .add_set(&pool, 23.0, 20.0, SetType::Normal)
            .await
            .expect("Failed to add warmup2 to counter rotation workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;
        counter_rotation_workout2
            .add_set(&pool, 23.0, 19.5, SetType::Normal)
            .await
            .expect("Failed to add warmup3 to counter rotation workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let running_workout2 = ExerciseWorkout::create(
            &pool,
            user.id.clone(),
            running.id.clone(),
            workout2.id.clone(),
        )
        .await
        .expect("Failed to add 'bench press' to 'workout2'");
        tokio::time::sleep(Duration::from_secs(1)).await;
        running_workout2
            .add_set(&pool, 1.0, 7.0 * 60.0 + 29.0, SetType::Normal)
            .await
            .expect("Failed to add normal1 to running workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let bench_press_workout2 = ExerciseWorkout::create(
            &pool,
            user.id.clone(),
            bench_press.id.clone(),
            workout2.id.clone(),
        )
        .await
        .expect("Failed to add 'bench press' to 'workout2'");
        tokio::time::sleep(Duration::from_secs(1)).await;
        bench_press_workout2
            .add_set(&pool, 20.0, 12.0, SetType::Warmup)
            .await
            .expect("Failed to add warmup1 to benchpress workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;
        bench_press_workout2
            .add_set(&pool, 30.0, 8.0, SetType::Normal)
            .await
            .expect("Failed to add normal1 to benchpress workout2");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
