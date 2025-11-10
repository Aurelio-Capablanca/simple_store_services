use std::{sync::Arc, vec};

use crate::inner::structures::service_structure::StateService;
use axum::{extract::State, response::Json};
use sqlx::{
    Row,
    types::chrono::{DateTime, Utc},
};

pub async fn tester_secured(State(state): State<Arc<StateService>>) -> Json<Vec<String>> {
    let connection = &state.database;
    let user_rows = sqlx::query("Select * from users")
        .fetch_all(connection)
        .await
        .unwrap();

    let mut fomatter: Vec<String> = Vec::new();
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
        fomatter.push(format!(
            "Users {},{},{},{},{},{},{},{},{}",
            id, name, email, verified, password, remember, created_at, updated_at, store_id
        ));
    }
    if fomatter.is_empty() {
        return Json(vec!["No Result Test".to_string()]);
    }
    Json(fomatter)
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
