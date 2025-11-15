use std::{sync::Arc, vec};
use crate::inner::services::product_service;
use crate::inner::structures::service_structure::{
    AuthenticatedUser, Categories, GeneralResponses, Identifier, LoadProduct, ProductRequest,
    ProductResponse, StateService,
};
use axum::response::IntoResponse;
use axum::{extract::State, response::Json};

use sqlx::types::BigDecimal;
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

pub async fn test_identities(
    State(state): State<Arc<StateService>>,
    AuthenticatedUser { id: user_id }: AuthenticatedUser,
) -> Json<Vec<String>> {
    let connection = &state.database;
    let user = sqlx::query("SELECT * FROM users where id = ?")
        .bind(user_id)
        .fetch_one(connection)
        .await
        .unwrap();
    let mut formatted: Vec<String> = Vec::new();

    let id: u64 = user.try_get("id").unwrap();
    let name: &str = user.try_get("name").unwrap();
    formatted.push(format!("User {}: {}", id, name));

    if formatted.is_empty() {
        Json(vec!["No Result".to_string()])
    } else {
        Json(formatted)
    }
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
//-------------------------------------------------------------------------------------------------------------------------------
//Action Controllers
pub async fn load_products_controller(
    State(state): State<Arc<StateService>>,
    Json(products): Json<LoadProduct>,
) -> impl IntoResponse {
    println!("Content Recieved : {:?}", products);
    let response = product_service::load_products(state, products).await;
    let done = match response {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            "".to_string()
        }
    };
    Json(done)
}

pub async fn get_categories_controller(
    State(state): State<Arc<StateService>>,
) -> GeneralResponses<Vec<Categories>> {
    let connection = &state.database;
    let results = sqlx::query(
        "SELECT id_category, category
            FROM simple_store.category;",
    )
    .fetch_all(connection)
    .await;

    let contents = match results {
        Ok(data) => data,
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    };

    let mut content_results: Vec<Categories> = Vec::new();
    for content in contents {
        let id: i64 = content.try_get("id_category").unwrap_or(0);
        let category: String = content.try_get("category").unwrap_or("N/F".to_string());
        let data = Categories {
            id_category: id,
            category: category,
        };
        content_results.push(data);
    }

    GeneralResponses {
        message: Some("Success".to_string()),
        dataset: Some(content_results),
        status: Some(0),
        error: Some("200".to_string()),
    }
}

pub async fn get_product_controller(
    State(state): State<Arc<StateService>>,
    Json(id): Json<Identifier>,
) -> GeneralResponses<ProductResponse> {
    let connection = &state.database;

    let fetch = sqlx::query(
        "
    SELECT 
        id_product, 
        product_name, 
        product_description, 
        product_price, 
        has_discount, 
        has_stock, 
        is_available, 
        expiring_date, 
        id_category, 
        product_stock_number, 
        is_discontinued
    FROM simple_store.product
    WHERE id_product = ?;
    ",
    )
    .bind(id.id)
    .fetch_one(connection)
    .await;

    let row = match fetch {
        Ok(result) => {
            println!("{:?}", result);
            result
        }
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    };
    let id = row.try_get("id_product").unwrap_or(0); //  id_product, 
    let name = row.try_get("product_name").unwrap_or("N/A").to_string(); //  product_name, 
    let description = row
        .try_get("product_description")
        .unwrap_or("N/A")
        .to_string(); //  product_description, 
    let price: BigDecimal = row.try_get("product_price").unwrap(); //  product_price, 
    let discount = row.try_get("has_discount").unwrap_or(false); //  has_discount, 
    let stock = row.try_get("has_stock").unwrap_or(false); //  has_stock, 
    let available = row.try_get("is_available").unwrap_or(false); //  is_available, 
    let expiration = row.try_get("expiring_date").unwrap_or("N/A").to_string(); //  expiring_date, 
    let category = row.try_get("id_category").unwrap_or(0); //  id_category, 
    let num_stock = row.try_get("product_stock_number").unwrap(); //  product_stock_number, 
    let discontinued = row.try_get("is_discontinued").unwrap_or(false); //  is_discontinued

    let content_results = ProductResponse {
        id_product: Some(id),
        product_name: Some(name),
        product_description: Some(description),
        product_price: Some(price),
        has_discount: Some(discount),
        has_stock: Some(stock),
        is_available: Some(available),
        expiring_date: Some(expiration),
        id_category: Some(category),
        product_stock_number: Some(num_stock),
        is_discontinued: Some(discontinued),
    };
    return GeneralResponses {
        message: Some("Success".to_string()),
        dataset: Some(content_results),
        status: Some(1),
        error: Some("200".to_string()),
    };
}

pub async fn update_product(
    State(state): State<Arc<StateService>>,
    Json(product): Json<ProductRequest>,
) -> GeneralResponses<String> {
    let response = product_service::update_product(state, product).await;
    match response {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    };

    return GeneralResponses {
        message: Some("Success".to_string()),
        dataset: None,
        status: Some(1),
        error: Some("200".to_string()),
    };
}

pub async fn delete_products_controller(
    State(state): State<Arc<StateService>>,
    Json(identifier): Json<Identifier>,
) -> GeneralResponses<String> {
    let result = product_service::logically_hid_products(state, identifier).await;

    match result {
        Ok(_) => {}
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("200".to_string()),
            };
        }
    }

    return GeneralResponses {
        message: Some("Success".to_string()),
        dataset: None,
        status: Some(1),
        error: Some("200".to_string()),
    };
}
