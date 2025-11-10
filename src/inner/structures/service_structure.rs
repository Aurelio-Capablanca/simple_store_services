use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

// System Required
pub struct StateService {
    pub database: MySqlPool,
    //add current User found
}

pub struct AuthenticatedUser {
    pub id : u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsJWT {
    iss: String,
    pub sub: u64,
    exp: u64,
    iat: u64,
}

//Business Structs
