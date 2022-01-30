use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
};
use headers::{Authorization, authorization::Bearer};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::error::AuthError;
use crate::jwt::Claims;

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