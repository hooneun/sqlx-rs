use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}


impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}