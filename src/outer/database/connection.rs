use std::time::Duration;

use sqlx::{Connection, MySqlConnection, MySqlPool, mysql::MySqlPoolOptions};


pub async fn up_connection() -> Result<MySqlPool, Box<dyn std::error::Error>>{
     let getter = sqlx::mysql::MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username("root")
        .password("jkl555")
        .database("simple_store");
    
    Ok(MySqlPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .connect_with(getter)
    .await?)
}