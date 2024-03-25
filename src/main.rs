use axum::Router;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

mod ctx;
mod dtos;
mod error;
mod helpers;
mod middlewares;
mod models;
mod routes;

#[derive(Clone)]
struct ApiState {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost/workout")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = ApiState { db: pool };
    let layer = Router::new();

    let app = layer
        .merge(routes::exercise::router(state.clone()))
        .merge(routes::auth::router(state.clone()));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
