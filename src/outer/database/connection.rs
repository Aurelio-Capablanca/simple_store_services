use sqlx::{Connection, MySqlConnection};


pub async fn up_connection() -> Result<MySqlConnection, Box<dyn std::error::Error>>{
     let getter = sqlx::mysql::MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username("root")
        .password("jkl555")
        .database("simple_store");

    let connection = sqlx::mysql::MySqlConnection::connect_with(&getter)
        .await
        .unwrap();
    Ok(connection)
}