
use axum::Router;
use models::{token::Token, user::User};
use mongodb::{
    options::ClientOptions,
    Client, Database,
};

mod dtos;
mod error;
mod helpers;
mod models;
mod routes;
mod middlewares;
mod ctx;

#[derive(Clone)]
struct ApiState {
    db: Database,
}

#[tokio::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.app_name = Some("AndrasWorkout".to_string());
    let client = Client::with_options(client_options).unwrap();

    let database = client.database("workout");

    Token::create_indexes(&database).await;
    User::create_indexes(&database).await;

    let state = ApiState { db: database };
    let layer = Router::new();

    let app = layer
        .merge(routes::auth::router(state));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
