use std::sync::Arc;
use axum::{Json, extract::State};
use crate::inner::{services::sales_services, structures::service_structure::{AuthenticatedUser, GeneralResponses, Sells, StateService}};


pub async fn do_sell_controller(
    State(state): State<Arc<StateService>>,
    AuthenticatedUser{id: user_id} : AuthenticatedUser,
    Json(sells) : Json<Sells>,    
)-> GeneralResponses<String> {

    let result = sales_services::do_sales(state, sells, user_id)
    .await;

    match result {
        Ok(_) => {},
        Err(err) => {
            return GeneralResponses {
                message: Some(format!("{} : {}", "Failure".to_string(), err)),
                dataset: None,
                status: Some(0),
                error: Some("500".to_string()),
            };
        }
    }

    GeneralResponses { message: Some("Success".to_string()),
     dataset: None, 
     status: Some(1), 
     error: None }
}



