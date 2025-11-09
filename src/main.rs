mod inner;
mod outer;

use axum::{Router, routing::get};

use tower_http::cors::CorsLayer;

use crate::{
    inner::{controllers::product_controller, structures::service_structure},
    outer::database::connection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let state_application = std::sync::Arc::new(service_structure::StateService {
        database: connection::up_connection().await.unwrap(),
    });
    print!("tester");
    let cors = CorsLayer::new().allow_headers([
        axum::http::header::AUTHORIZATION,
        axum::http::header::ACCEPT,
        axum::http::header::CONTENT_TYPE,
    ]);
    let application : Router = Router::new()
        .route("/", get(product_controller::hello_world))
        .with_state(state_application)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9091").await.unwrap();
    axum::serve(listener, application).await.unwrap();
    Ok(())
}
