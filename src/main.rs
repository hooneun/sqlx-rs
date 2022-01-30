use std::net::SocketAddr;

use sqlx::MySqlPool;

mod configs;
mod error;
mod extractors;
mod handlers;
mod jwt;
mod models;
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = configs::db::DB::url();

    let pool = MySqlPool::connect(&database_url).await.unwrap();

    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router::router(pool).into_make_service())
        .await
        .unwrap();
}
