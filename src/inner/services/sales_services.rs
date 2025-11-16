use crate::inner::structures::service_structure::{ Sells, StateService};
use sqlx::Row;
use std::sync::Arc;

pub async fn do_sales(
    state: Arc<StateService>,
    sell: Sells,
    id: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut transaction = state.database.begin().await?;
    //Check the existence of the User
    let fetch_seller = sqlx::query(
        "
    SELECT id
    FROM simple_store.users
    WHERE id = ?;
    ",
    )
    .bind(id)
    .fetch_one(&mut *transaction)
    .await;

    let id_user: u64 = match fetch_seller {
        Ok(result) => {
            result.try_get("id").unwrap()
        }
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    };

    //Insert a Sell
    /*INSERT INTO simple_store.sells
    (id_sell, id_product, person_in_charge, id_store)
    VALUES(0, CURRENT_TIMESTAMP, 0, 0, 0);*/
    let executor_one = sqlx::query(
        "
        INSERT INTO simple_store.sells
        (person_in_charge, id_store)
        VALUES( ?, ?);
        ",
    )
    .bind(id_user)
    .bind(&sell.id_store)
    .execute(&mut *transaction)
    .await;

    let id_sell: u64 = match executor_one {
        Ok(result) => {
            result.last_insert_id()
        }
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    };

    let mut total_sold: f64 = 0f64;
    //Product Sold
    for id_product in &sell.products {
        total_sold += &id_product.sell_price.unwrap_or(0f64);
        /*INSERT INTO simple_store.products_sold
        (id_product, id_sell, sell_price, total_cart)
        VALUES(?, ?, ?, ?);*/
        let executor_two = sqlx::query(
            "
        INSERT INTO simple_store.products_sold
        (id_product, id_sell, sell_price, total_cart)
        VALUES(?, ?, ?, ?);
        ",
        )
        .bind(&id_product.id_product)
        .bind(id_sell)
        .bind(&id_product.sell_price.unwrap_or(0f64))
        .bind(&id_product.total_cart)
        .execute(&mut *transaction)
        .await;
        match executor_two {
            Ok(_) => {}
            Err(err) => {
                transaction.rollback().await?;
                return Err(Box::new(err));
            }
        }
    }

    //Update Total_Capital
    /* UPDATE simple_store.store
    SET total_capital= total_capital + ?
    WHERE id_store=?; */
    let executor_three = sqlx::query(
        "
    UPDATE simple_store.store
    SET total_capital= total_capital + ?
    WHERE id_store=?;
    ",
    )
    .bind(&sell.id_store)
    .execute(&mut *transaction)
    .await;

    match executor_three {
        Ok(_) => {},
        Err(err) => {
            transaction.rollback().await?;
            return Err(Box::new(err));
        }
    }

    transaction.commit().await?;

    Ok(true)
}

