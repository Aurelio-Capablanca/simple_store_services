use sqlx::{MySqlPool};

// System Required
pub struct StateService {  
    pub database: MySqlPool
}

pub struct ClaimsJWT{
    
    exp: u64
}

//Business Structs 
