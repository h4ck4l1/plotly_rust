#![allow(unused)]

use std::sync::Arc;
use axum::{http::HeaderValue, routing, Router};
use backend::{mushroom::mushroom_handler,BackendError,AppState,ALL_FIT_JSON,MUSHROOM};
use hyper::Method;
use polars::prelude::all;
use tower_http::cors::{Any, CorsLayer};

const IS_PRODUCTION: bool = false;

#[tokio::main]
async fn main() -> Result<(),BackendError> {
    
    tracing_subscriber::fmt::init();

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

    let app = if !IS_PRODUCTION {
        let cors_layer = CorsLayer::new()
            .allow_headers(Any)
            .allow_origin(Any)
            .allow_methods(Any);
        Router::new()
        .route("/api/mushroom", routing::get(mushroom_handler))
        .with_state(app_state)
        .layer(cors_layer)
    } else {
        Router::new()
        .route("/api/mushroom", routing::get(mushroom_handler))
        .with_state(app_state)
    };


    let listener = tokio::net::TcpListener::bind(
        "[::]:3000"
    ).await?;

    tracing::info!("Server running on : {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
