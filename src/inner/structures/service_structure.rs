use serde::{Deserialize, Serialize};
use sqlx::{
    MySqlPool,
    types::chrono::{DateTime, Local, NaiveDateTime, Utc},
};

// System Required
pub struct StateService {
    pub database: MySqlPool,
    //add current User found
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

//Database Paired Structs
#[derive(Debug, Clone)]
struct RetailerBill {
    id_retailer_bill: Option<i64>,
    amount_billed: f64,
    timestap_bill_retailer: DateTime<Local>,
    id_store: i32,
    id_retailer: i32,
}

struct ProductBill{
    id_product : i64,
    id_bill: i64
}

#[derive(Debug, Clone)]
struct Product {
    id_product: Option<i32>,
    product_name: String,
    product_description: String,
    product_price: f64,
    has_discount: Option<bool>,
    has_stock: Option<bool>,
    is_available: Option<bool>,
    expiring_date: Option<DateTime<Local>>,
    id_category: i32,
    buying_price: f64,
    unique_code: String,
    product_stock_number: i64,
    is_discontinued : bool
}

//Request Payloads
