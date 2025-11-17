use crate::inner::{
    services::sales_services,
    structures::service_structure::{
        AuthenticatedUser, GeneralResponses, Identifier, Sells, SoldProducts, StateService
    },
};
use axum::{Json, extract::State};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use sqlx::Row;

pub async fn do_sell_controller(
    State(state): State<Arc<StateService>>,
    AuthenticatedUser { id: user_id }: AuthenticatedUser,
    Json(sells): Json<Sells>,
) -> GeneralResponses<String> {
    let result = sales_services::do_sales(state, sells, user_id).await;

    match result {
        Ok(_) => {}
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    }

    GeneralResponses {
        message: Some("Success".to_string()),
        dataset: None,
        status: Some(1),
        error: None,
    }
}

pub async fn disable_sell(
    State(state): State<Arc<StateService>>,
    Json(identifier): Json<Identifier>,
) -> GeneralResponses<String> {
    let mut transaction = match state.database.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    };

    let executor = sqlx::query(
        "
        UPDATE simple_store.sells
        SET is_deactivated = 1
        WHERE id_sell=?;
        ",
    )
    .bind(identifier.id)
    .execute(&mut *transaction)
    .await;

    match executor {
        Ok(_) => {}
        Err(err) => {
            match transaction.rollback().await {
                Ok(tx) => tx,
                Err(err) => {
                    return GeneralResponses {
                        message: Some(format!("{} : {}", "Failure".to_string(), err)),
                        dataset: None,
                        status: Some(0),
                        error: Some("500".to_string()),
                    };
                }
            };

            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    }

    match transaction.commit().await {
        Ok(tx) => tx,
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    };

    GeneralResponses {
        message: Some("Success".to_string()),
        dataset: None,
        status: Some(1),
        error: None,
    }
}

pub async fn get_sells_products(
    State(state): State<Arc<StateService>>,
) -> GeneralResponses<Vec<SoldProducts>> {
    let connection = &state.database;

    let execution = sqlx::query(
        "
        select sll.id_sell, sll.timestamp_sell, SUM(ps.sell_price) as total_sold, u.name ,GROUP_CONCAT(pr.product_name SEPARATOR ', ') AS all_products
        from sells sll
        inner join products_sold ps  on sll.id_sell  = ps.id_sell 
        inner join product pr on ps.id_product  = pr.id_product 
        inner join users u on u.id  = sll.person_in_charge 
        group by sll.id_sell
    ",
    )
    .fetch_all(connection)
    .await;

    match execution {
        Ok(rows) => {
            let mut rows_send : Vec<SoldProducts> = Vec::new();
            for row in rows {
                let id_sell : i64 = row.try_get("id_sell").unwrap();
                let timestamp_sell : DateTime<Utc> = row.try_get("timestamp_sell").unwrap();
                let total_sold : BigDecimal = row.try_get("total_sold").unwrap();
                let employee : String = row.try_get("name").unwrap();
                let products : String = row.try_get("all_products").unwrap();
                let final_timestamp = timestamp_sell.format("%Y-%m-%d  %H:%M:%S").to_string();
                let product_sold : SoldProducts = SoldProducts { id_sold: id_sell, time_stamp: final_timestamp, products, total_sold, in_charge_name: employee };
                rows_send.push(product_sold);
            }            
            return GeneralResponses {
                message: Some("Success".to_string()),
                dataset: Some(rows_send),
                status: Some(1),
                error: None,
            };
        }
        Err(err) => {            
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    }
}
