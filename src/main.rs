use std::net::SocketAddr;
use axum::{AddExtensionLayer, Router, response::Json};
use axum::extract::Extension;
use axum::routing::get;
use sqlx::{postgres::PgPoolOptions, MySqlPool, Pool, MySql, PgPool};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(database_url)
    //     .await?;
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL Not found");

    let pool = MySqlPool::connect(&database_url).await.unwrap();

    tracing_subscriber::fmt::init();

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(pool.to_owned()))
        .into_inner();

    let app = Router::new()
        .route("/", get(index))
        .layer(middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    Extension(pool): Extension<Pool<MySql>>
) -> Json<Value> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE login_id = ?")
        .bind("solomon2")
        .fetch_one(&pool)
        .await;

    match user {
        Ok(user) => {
            Json(json!(user))
        },
        Err(_) => Json(json!("")),
    }
}

// async fn login(
//     Json<input>: Json<LoginInput>,
//     Extension<pool>: Extension<PgPool>,
// ) {
//
// }

#[derive(Debug)]
struct LoginInput {
    login_id: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    user_id: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct User {
    id: u64,
    name: String,
}