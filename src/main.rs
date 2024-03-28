use std::io;

use axum::{routing::get_service, Router};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use tower_http::{cors::CorsLayer, services::ServeDir};

mod ctx;
mod dtos;
mod error;
mod helpers;
mod middlewares;
mod models;
mod routes;
mod seeder;
mod response;

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

    let state = ApiState { db: pool };
    let layer = Router::new();

    let app = layer
        .merge(routes::exercise::router(state.clone()))
        .merge(routes::workout::router(state.clone()))
        .merge(routes::set::router(state.clone()))
        .merge(routes::auth::router(state.clone()))
        .nest_service("/", get_service(ServeDir::new("./static"))).
        layer(CorsLayer::permissive());

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started");
    axum::serve(listner, app).await.unwrap();
}
