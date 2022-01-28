use headers::{authorization::Bearer, Authorization};

#[async_trait]
impl <B> FromRequest<B> for User {
    
}


// #[async_trait]
// impl<B> FromRequest<B> for Claims
//     where
//         B: Send,
// {
//     type Rejection = AuthError;
//
//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         // Extract the token from the authorization header
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req)
//                 .await
//                 .map_err(|_| AuthError::InvalidToken)?;
//         // Decode the user data
//         let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//
//         Ok(token_data.claims)
//     }
// }
