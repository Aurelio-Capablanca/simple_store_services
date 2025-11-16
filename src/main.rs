mod inner;
mod outer;

use crate::{
    inner::{
        controllers::{product_controller, sells_controller},
        structures::service_structure::{self},
    },
    outer::{
        database::connection,
        security::jwt_middleware::{self},
    },
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;

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

    let public_routes = Router::new().route("/", get(product_controller::hello_world));

    let private_routes = Router::new()
        .route("/test-access", get(product_controller::tester_secured))
        .route("/test-identity", get(product_controller::test_identities))
        //Products
        .route(
            "/load-products",
            post(product_controller::load_products_controller),
        )
        .route(
            "/get-categories",
            get(product_controller::get_categories_controller),
        )
        .route(
            "/get-one-product",
            post(product_controller::get_product_controller),
        )
        .route("/update-products", put(product_controller::update_product))
        .route(
            "/delete-product",
            delete(product_controller::delete_products_controller),
        )
        //Sales
        .route("/create-sale", post(sells_controller::do_sell_controller))
        .route_layer(middleware::from_fn(jwt_middleware::jwt_middleware))
        .with_state(state_application);

    let application: Router = Router::new()
        .merge(public_routes)
        .nest("/api", private_routes)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9091")
        .await
        .unwrap();
    axum::serve(listener, application).await.unwrap();
    Ok(())
}
