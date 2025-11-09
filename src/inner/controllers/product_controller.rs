use std::sync::Arc;

use axum::extract::State;
use sqlx::{
    Row,
    types::chrono::{DateTime, Utc},
};
use crate::inner::structures::service_structure::StateService;

pub async fn hello_world(State(state): State<Arc<StateService>>) -> &'static str {    
    let connection = &state.database;
    let user_rows = sqlx::query("Select * from users")
        .fetch_all(connection)
        .await
        .unwrap();

    for row in user_rows {
        let id: u64 = row.try_get("id").unwrap();
        let name: &str = row.try_get("name").unwrap();
        let email: &str = row.try_get("email").unwrap();
        let verified: DateTime<Utc> = row.try_get("email_verified_at").unwrap_or(Utc::now());
        let password: &str = row.try_get("password").unwrap();
        let remember: &str = row.try_get("remember_token").unwrap_or("Nothing to see");
        let created_at: DateTime<Utc> = row.try_get("created_at").unwrap();
        let updated_at: DateTime<Utc> = row.try_get("updated_at").unwrap();
        let store_id: i32 = row.try_get("id_store").unwrap();
        println!(
            "Users {},{},{},{},{},{},{},{},{}",
            id, name, email, verified, password, remember, created_at, updated_at, store_id
        );
    }
    "Hello, world!"
}
