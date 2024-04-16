use std::{io, time::Duration};

use axum::{routing::get_service, Router};
use futures::FutureExt;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::models::token::Token;

mod ctx;
mod dtos;
mod error;
mod helpers;
mod middlewares;
mod models;
mod response;
mod routes;
mod seeder;

#[derive(Clone)]
struct ApiState {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        // env variables might come from os env
        Err(dotenvy::Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => (),
        Err(err) => panic!("{}", err),
        Ok(_) => (),
    }

    let database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL present");
    println!("{database_url}");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = ApiState { db: pool.clone() };
    let layer = Router::new();

    tokio::spawn(async move {
        println!("Starting deleting expired tokens task");

        let mut interval = tokio::time::interval(Duration::from_secs(60 * 60));

        loop {
            interval.tick().await;

            match Token::delete_expired(&pool).await {
                Err(err) => println!("Failed to delete expired tokens {:#?}", err),
                Ok(_) => (),
            }
        }
    });

    let app = layer
        .merge(routes::exercise::router(state.clone()))
        .merge(routes::workout::router(state.clone()))
        .merge(routes::set::router(state.clone()))
        .merge(routes::auth::router(state.clone()))
        .merge(routes::target::router(state.clone()))
        .nest_service("/", get_service(ServeDir::new("./static")))
        .layer(CorsLayer::permissive());

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started");
    axum::serve(listner, app).await.unwrap();
}
