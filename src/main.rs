use std::net::SocketAddr;

use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use error::AuthError;
use jwt::Claims;

mod error;
mod extractors;
mod handlers;
mod jwt;
mod models;
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL Not found");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router::router(pool).into_make_service())
        .await
        .unwrap();
}

#[derive(Debug)]
struct LoginInput {
    login_id: String,
    password: String,
}