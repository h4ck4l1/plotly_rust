#![allow(unused)]

use std::sync::Arc;
use axum::{http::HeaderValue, routing, Router};
use backend::{mushroom_pages::mushroom_cap_diameter::mushroom_cap_diameter, AppState, BackendError, ALL_FIT_JSON, MUSHROOM};
use hyper::Method;
use polars::prelude::all;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() -> Result<(),BackendError> {
    
    tracing_subscriber::fmt::init();


    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(["http://localhost:8080".parse::<HeaderValue>().unwrap()])
        .allow_headers(Any);

    let app_state = Arc::new(AppState::new(

        // Apply Appwide Dataframe transformations here
        
        MUSHROOM
            .to_owned()
            .select([all().cast(polars::prelude::DataType::Float32)])
            .collect()?,

        // Lazy initialization of json data

        ALL_FIT_JSON
            .to_owned()
        )
    );

    let app = Router::new()
        .route("/mushroom_cap_diameter", routing::get(mushroom_cap_diameter))
        .with_state(app_state)
        .layer(cors_layer);


    let listener = tokio::net::TcpListener::bind(
        std::net::SocketAddr::from(([127,0,0,1],3000))
    ).await?;

    tracing::info!("Server running on : {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}