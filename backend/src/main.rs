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

    dotenvy::dotenv().ok();
    
    let filename = if IS_PRODUCTION {
        format!(".env.production")
    } else {
        format!(".env.development")
    };
    dotenvy::from_filename(&filename)
        .unwrap_or_else(|_| panic!("Failed to load environment"));

    let backend_api = std::env::var("API_BASE")
        .unwrap_or_else(|_| panic!("unable to get environment variable"));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin([(&backend_api).parse::<HeaderValue>().unwrap()])
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
        .route("/api/mushroom", routing::get(mushroom_handler))
        .with_state(app_state)
        .layer(cors_layer);


    let listener = tokio::net::TcpListener::bind(
        std::net::SocketAddr::from(([127,0,0,1],3000))
    ).await?;

    tracing::info!("Server running on : {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}