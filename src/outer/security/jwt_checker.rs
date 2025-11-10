use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::inner::structures::service_structure;

pub async fn jwt_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {   
    let auth_headers = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    println!("{:?}", auth_headers);
    let Some(auth_header) = auth_headers else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let secret_keys = b"791376c27ad90e5594339a004d26ef259e8faaba";
    let decoded = DecodingKey::from_secret(secret_keys);
    println!("Decode : {:?}", decoded);
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    println!("Validation : {:?}", validation);
    let data_token = decode::<service_structure::ClaimsJWT>(token, &decoded, &validation);
    println!("Data : {:?}", data_token);
    match data_token {
        Ok(data) => {
            request.extensions_mut().insert(data.claims);
            Ok(next.run(request).await)
        }
        Err(_) => {
            print!("Trapped in error!");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
