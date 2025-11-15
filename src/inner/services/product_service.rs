use sqlx::types::chrono::{NaiveDate, Utc};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::inner::structures::service_structure::{
    Identifier, LoadProduct, ProductRequest, StateService,
};

struct ProductIntermediary {
    id: u64,
    buy_price: f64,
}

// Save Product (as stated on diagrams)
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
    let mut total_bill_due: f64 = 0f64;
    let product_list = loader.list_product;
    let mut product_intermediary: Vec<ProductIntermediary> = Vec::with_capacity(product_list.len());

    for product in &product_list {
        total_bill_due += &product.buying_price.unwrap_or(0f64);
    }

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

    let seek_store = sqlx::query(
        "
    SELECT id_store
    FROM simple_store.store
    Where id_store = ?;
    ",
    )
    .bind(&retailer_data.id_store)
    .fetch_one(&mut *transaction)
    .await;

    match seek_store {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    // Insert retailer bill
    let executioner_one = sqlx::query(
        "INSERT INTO simple_store.retailer_bill
        (amount_billed, timestap_bill_retailer,id_store, id_retailer)
        VALUES(?, ?, ?, ?);",
    )
    .bind(&total_bill_due)
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

    for product in product_list {
        let buy_price_product = product.buying_price.unwrap_or(0f64);
        //if the Id is present don't make inserts, ignore and left over the object
        if let Some(id_product_or) = product.id_product {
            let id = match u64::try_from(id_product_or) {
                Ok(res) => res,
                Err(err) => {
                    transaction.rollback().await?;
                    return Err(Box::new(err));
                }
            };
            product_intermediary.push(ProductIntermediary {
                id: id,
                buy_price: buy_price_product,
            });
            continue;
        }

        let name = product.product_name.unwrap_or("No Name".to_string());
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

        let random = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
            .to_string();
        let prefix: String = name.chars().take(3).collect();
        let code = format!("{:?}-{:?}", prefix, random);

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
        .bind(&name) //product_name,
        .bind(
            &product
                .product_description
                .unwrap_or("no Description".to_string())
                .to_string(),
        ) //product_description
        .bind(&product.product_price.unwrap_or(0f64))
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

        product_intermediary.push(ProductIntermediary {
            id: id_product,
            buy_price: buy_price_product,
        });
    }

    for product in product_intermediary {
        //Product Bill
        //---------------
        /*INSERT INTO simple_store.product_bill
        (id_product, id_bill, buying_price)
        VALUES(0, 0, 0); */
        let executioner_three = sqlx::query(
            "INSERT INTO simple_store.product_bill
                (id_product, id_bill, buying_price)
                VALUES(?, ?, ?);",
        )
        .bind(&product.id)
        .bind(&id_bill_retailer)
        .bind(&product.buy_price)
        .execute(&mut *transaction)
        .await;

        match executioner_three {
            Ok(_) => {
                print!("goes right!")
            }
            Err(err) => {
                transaction.rollback().await?;
                return Err(Box::new(err));
            }
        }
        //Product to Store
        //---------------
        /*INSERT INTO simple_store.product_location
        (id_product, id_store)
        VALUES(0, 0); */
        let executioner_four = sqlx::query(
            "INSERT INTO simple_store.product_location
            (id_product, id_store)
            VALUES(?, ?);",
        )
        .bind(&product.id)
        .bind(&retailer_data.id_store)
        .execute(&mut *transaction)
        .await;

        match executioner_four {
            Ok(_) => {}
            Err(err) => {
                transaction.rollback().await?;
                return Err(Box::new(err));
            }
        }
    }

    //Update Total_capital at Store
    /*UPDATE simple_store.store
    SET store_number=0, store_location=0, total_capital=0, store_name=''
    WHERE id_store=0; */
    let executor_five = sqlx::query(
        "
    UPDATE simple_store.store
    SET total_capital= total_capital - ?
    WHERE id_store=?;
    ",
    )
    .bind(&total_bill_due)
    .bind(&retailer_data.id_store)
    .execute(&mut *transaction)
    .await;

    match executor_five {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    transaction.commit().await?;

    Ok("Success".to_string())
}

//Save those individually, don't wrestle up with this!
pub async fn update_product(
    database: Arc<StateService>,
    product: ProductRequest,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut transaction = database.database.begin().await?;

    let id = product.id_product.unwrap_or(0);
    /*SELECT id_product FROM simple_store.product WHERE id_product = ?; */
    let exist_p_q = sqlx::query("SELECT id_product FROM simple_store.product WHERE id_product = ?")
        .bind(id)
        .fetch_one(&mut *transaction)
        .await;

    match exist_p_q {
        Ok(_) => {}
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    }

    /*UPDATE simple_store.product
    SET product_name='', product_description='', product_price=0, has_discount=b'0',
    has_stock=b'0', is_available=b'0', expiring_date='', id_category=0,
    unique_code='', product_stock_number=0, is_discontinued=0
    WHERE id_product=0; */
    let name = &product
        .product_name
        .unwrap_or("N/P".to_string())
        .to_string();
    let description = &product.product_description.unwrap_or("N/P".to_string());
    let price = &product.product_price.unwrap_or(0f64);
    let discount = if product.has_discount.unwrap_or(false) {
        1u8
    } else {
        0u8
    };
    let has_stock = if product.has_stock.unwrap_or(false) {
        1u8
    } else {
        0u8
    };
    let available = if product.is_available.unwrap_or(false) {
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
    let category = product.id_category.unwrap_or(0);
    let stock = product.product_stock_number.unwrap_or(0);

    let executor = sqlx::query(
        "
    UPDATE simple_store.product
    SET product_name=?, 
    product_description=?, 
    product_price=?, 
    has_discount=?,
    has_stock=?, 
    is_available=?,
    expiring_date=?,
    id_category=?,
    product_stock_number=?
    WHERE id_product=?;
    ",
    )
    .bind(name)
    .bind(description)
    .bind(price)
    .bind(discount)
    .bind(has_stock)
    .bind(available)
    .bind(date_expiration)
    .bind(category)
    .bind(stock)
    .bind(id)
    .execute(&mut *transaction)
    .await;

    match executor {
        Ok(_) => {}
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    }

    transaction.commit().await?;

    Ok(true)
}

pub async fn logically_hid_products(
    state: Arc<StateService>,
    identificator: Identifier,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut transaction = state.database.begin().await?;
    
    /*UPDATE simple_store.product
    SET is_discontinued=0
    WHERE id_product=?; */
    let execution = sqlx::query(
        "
        UPDATE simple_store.product
        SET is_discontinued=0
        WHERE id_product=?;
        "
    )
    .bind(identificator.id)
    .execute(&mut *transaction)    
    .await;

    match execution {
        Ok(_) => {
            println!("Product deactivated!")
        },
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    }

    transaction.commit().await?;

    Ok(true)
}
