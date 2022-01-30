use axum::extract::Extension;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use sqlx::{MySql, Pool};

use crate::{
    error::AuthError,
    jwt::Claims,
    models::{
        auth::{AuthBody, AuthPayload},
        user::User,
    },
};

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

pub async fn non_protected() -> Result<String, AuthError> {
    Ok(format!("Welcome to the non protected area :)"))
}

pub async fn index(Extension(pool): Extension<Pool<MySql>>) -> Json<Value> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE login_id = ?")
        .bind("solomon2")
        .fetch_one(&pool)
        .await;

    match user {
        Ok(user) => Json(json!(user)),
        Err(_) => Json(json!("")),
    }
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.user_id.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let id: u64 = 444;
    let claims = Claims::new(id);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}
