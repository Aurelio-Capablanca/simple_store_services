use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{
    MySql,  Pool, types::BigDecimal
};

// System Required
pub struct StateService {
    pub database: Pool<MySql>
}

#[derive(Serialize, Debug)]
pub struct GeneralResponses<T> {
    pub message: Option<String>,
    pub dataset: Option<T>,
    pub status: Option<i32>,
    pub error: Option<String>
}

impl<T: serde::Serialize> IntoResponse for GeneralResponses<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}


pub struct AuthenticatedUser {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsJWT {
    iss: String,
    pub sub: u64,
    exp: u64,
    iat: u64,
}

//Request Payloads
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductRequest {
    pub id_product: Option<i32>,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_price: Option<f64>,
    pub has_discount: Option<bool>,
    pub has_stock: Option<bool>,
    pub is_available: Option<bool>,
    pub expiring_date: Option<String>,
    pub id_category: Option<i32>,
    pub buying_price: Option<f64>,
    pub unique_code: Option<String>,
    pub product_stock_number: Option<i64>,
    pub is_discontinued : Option<bool>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identifier{
    pub id : u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetailerBillRequest {
    pub id_retailer_bill: Option<i64>,
    pub amount_billed: Option<f64>,
    pub timestap_bill_retailer: Option<String>,
    pub id_store: Option<i32>,
    pub id_retailer: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadProduct{
    pub retailer_bill : RetailerBillRequest,
    pub list_product: Vec<ProductRequest>
}


//Response Payloads

#[derive(Serialize)]
pub struct Categories{
    pub id_category: i64,
    pub category: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResponse {
    pub id_product: Option<i32>,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_price: Option<BigDecimal>,
    pub has_discount: Option<bool>,
    pub has_stock: Option<bool>,
    pub is_available: Option<bool>,
    pub expiring_date: Option<String>,
    pub id_category: Option<i32>,        
    pub product_stock_number: Option<i64>,
    pub is_discontinued : Option<bool>
}
