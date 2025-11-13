use sqlx::Executor;
use sqlx::{
    MySql, Transaction, query,
    types::chrono::{NaiveDate, Utc},
};
use std::sync::Arc;

use crate::inner::structures::service_structure::{LoadProduct, RetailerBill, StateService};

pub async fn load_products(
    state: Arc<StateService>,
    loader: LoadProduct,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("reaching Services");
    //let connection = &state.database;
    //Insert Retailer Bill
    /*INSERT INTO simple_store.retailer_bill
    (id_retailer_bill, amount_billed, timestap_bill_retailer,
    id_store, id_retailer)
    VALUES(0, 0, '', 0, 0); */
    let retailer_data = &loader.retailer_bill;
    let message_error: String;
    let status: bool;

    let date_bill = match NaiveDate::parse_from_str(
        retailer_data
            .timestap_bill_retailer
            .as_deref()
            .unwrap_or(Utc::now().format("%Y-%m-%d").to_string().as_str()),
        "%Y-%m-%d",
    ) {
        Ok(date) => date.and_hms_opt(0, 0, 0).unwrap(),
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    let mut transaction = state.database.begin().await?;

    // Insert retailer bill
    let executioner_one = sqlx::query(
        "INSERT INTO simple_store.retailer_bill
        (amount_billed, timestap_bill_retailer,id_store, id_retailer)
        VALUES(?, ?, ?, ?);",
    )
    .bind(&retailer_data.amount_billed)
    .bind(date_bill)
    .bind(&retailer_data.id_store)
    .bind(&retailer_data.id_retailer)
    .execute(&mut *transaction)
    .await;
    let id_bill_retailer = match executioner_one {
        Ok(res) => res.last_insert_id(),
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    };

    // Insert Products
    /*INSERT INTO simple_store.product
    (id_product, product_name, product_description, product_price,
    has_discount, has_stock, is_available, expiring_date, id_category,
     unique_code, product_stock_number, is_discontinued)
    VALUES(0, '', '', 0, b'0', b'0', b'0', '', 0, '', 0, 0); */
    let product_list = loader.list_product;
    product_list.iter().for_each(|product| {
        
    });

    let executioner_two = sqlx::query(
        "INSERT INTO simple_store.product
    (product_name, 
    product_description, 
    product_price,
    has_discount, 
    has_stock, 
    is_available, 
    expiring_date, 
    id_category,
    unique_code, 
    product_stock_number, 
    is_discontinued)
    VALUES(?, ?, ?, ?, ?, ?, ?,?, ?,?,?);",
    )
    .execute(&mut *transaction)
    .await;

    Ok(String::new())
}
