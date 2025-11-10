mod inner;
mod outer;

use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use tower_http::cors::CorsLayer;

use crate::{
    inner::{controllers::product_controller, structures::service_structure::{self, StateService}},
    outer::{database::connection, security::jwt_checker::{self}},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let state_application = std::sync::Arc::new(service_structure::StateService {
        database: connection::up_connection().await.unwrap(),
    });
    println!("tester");
    let cors = CorsLayer::new().allow_headers([
        axum::http::header::AUTHORIZATION,
        axum::http::header::ACCEPT,
        axum::http::header::CONTENT_TYPE,
    ]);

    let public_routes: Router<Arc<StateService>> = Router::new()
    .route("/", get(product_controller::hello_world));

    let private_routes : Router<Arc<StateService>> = Router::new()
    .route("/test-access", get(product_controller::tester_secured))
    .route_layer(middleware::from_fn(jwt_checker::jwt_middleware));

    let application : Router = Router::new()
        .merge(public_routes)
        .nest("/api", private_routes)
        .with_state(state_application)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9091").await.unwrap();
    axum::serve(listener, application).await.unwrap();
    Ok(())
}
