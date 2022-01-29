use std::fmt::Display;
use std::net::SocketAddr;

use axum::{
    AddExtensionLayer,
    async_trait, extract::{Extension, FromRequest, RequestParts, TypedHeader},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
    Json,
};
use chrono::{Duration, Utc};
use headers::{Authorization, authorization::Bearer};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{MySql, MySqlPool, PgPool, Pool};
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
        .route("/authorize", post(authorize))
        .route("/protected", get(protected))
        .layer(middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

async fn index(Extension(pool): Extension<Pool<MySql>>) -> Json<Value> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE login_id = ?")
        .bind("solomon2")
        .fetch_one(&pool)
        .await;

    match user {
        Ok(user) => Json(json!(user)),
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

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: u64,
    exp: i64,
    iat: i64,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId: {}", self.sub)
    }
}

impl Claims {
    pub fn new(id: u64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.user_id.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let id: u64 = 444;
    let claims = Claims::new(id);

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_bytes()))
        .map_err(|_| AuthError::TokenCreation)?;


    Ok(Json(AuthBody::new(token)))
}

#[async_trait]
impl<B> FromRequest<B> for Claims
    where
        B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &DecodingKey::from_secret("secret".as_bytes()), &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}