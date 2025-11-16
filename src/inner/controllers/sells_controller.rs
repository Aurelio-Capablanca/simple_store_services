use crate::inner::{
    services::sales_services,
    structures::service_structure::{
        AuthenticatedUser, GeneralResponses, Identifier, Sells, StateService,
    },
};
use axum::{Json, extract::State};
use std::sync::Arc;

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
