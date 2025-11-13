use sqlx::types::chrono::{NaiveDate, Utc};
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use crate::inner::structures::service_structure::{LoadProduct, StateService};

pub async fn load_products(
    state: Arc<StateService>,
    loader: LoadProduct,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("reaching Services");

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
    let mut product_ids: Vec<u64> = Vec::with_capacity(product_list.len());

    for product in product_list {
        let name = &product.product_name.unwrap_or("No Name".to_string());
        let has_discount = if product.has_discount.unwrap_or(false) {
            1u8
        } else {
            0u8
        };
        let has_stock = if product.has_stock.unwrap_or(false) {
            1u8
        } else {
            0u8
        };
        let is_available = if product.is_available.unwrap_or(false) {
            1u8
        } else {
            0u8
        };

        let date_expiration = match NaiveDate::parse_from_str(
            product
                .expiring_date
                .as_deref()
                .unwrap_or(Utc::now().format("%Y-%m-%d").to_string().as_str()),
            "%Y-%m-%d",
        ) {
            Ok(date) => date.and_hms_opt(0, 0, 0).unwrap(),
            Err(err) => {
                return Err(Box::new(err));
            }
        };

        let is_discontinued = if product.is_discontinued.unwrap_or(false) {
            1u8
        } else {
            0u8
        };

        let random = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string();
        let prefix : String = name.chars().take(3).collect();
        let code = format!("{:?}-{:?}",prefix,random);

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
        .bind(&name)
        .bind(
            &product
                .product_description
                .unwrap_or("no Description".to_string())
                .to_string(),
        )
        .bind(has_discount)
        .bind(has_stock)
        .bind(is_available)
        .bind(date_expiration)
        .bind(&product.id_category.unwrap_or(0))
        .bind(code)
        .bind(&product.product_stock_number.unwrap_or(0))
        .bind(is_discontinued)
        .execute(&mut *transaction)
        .await;

        let id_product = match executioner_two {
            Ok(row) => row.last_insert_id(),
            Err(err) => {
                transaction.rollback().await?;
                return Err(Box::new(err));
            }
        };
        product_ids.push(id_product);
    }

    for id_product in product_ids {

        //Product Bill
        //---------------
        /*INSERT INTO simple_store.product_bill
        (id_product, id_bill, buying_price)
        VALUES(0, 0, 0); */

        //Product to Store
        //---------------
        /*INSERT INTO simple_store.product_location
        (id_product, id_store)
        VALUES(0, 0); */
    }

    transaction.commit().await?;

    Ok(String::new())
}
