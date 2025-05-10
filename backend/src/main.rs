#![allow(unused)]

use std::sync::Arc;
use axum::{http::HeaderValue, routing, Router};
use backend::{mushroom_pages::{mushroom_cap_diameter::mushroom_cap_diameter, mushroom_cap_shape::mushroom_cap_shape, mushroom_class::mushroom_class, mushroom_gill_attachment::mushroom_gill_attachment, mushroom_gill_color::mushroom_gill_color, mushroom_season::mushroom_season, mushroom_stem_color::mushroom_stem_color, mushroom_stem_height::mushroom_stem_height, mushroom_stem_width::mushroom_stem_width}, AppState, BackendError, ALL_FIT_JSON, MUSHROOM};
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
        .route("/api/mushroom_cap_diameter", routing::get(mushroom_cap_diameter))
        .route("/api/mushroom_cap_shape", routing::get(mushroom_cap_shape))
        .route("/api/mushroom_gill_attachment", routing::get(mushroom_gill_attachment))
        .route("/api/mushroom_gill_color", routing::get(mushroom_gill_color))
        .route("/api/mushroom_class", routing::get(mushroom_class))
        .route("/api/mushroom_stem_color", routing::get(mushroom_stem_color))
        .route("/api/mushroom_stem_height", routing::get(mushroom_stem_height))
        .route("/api/mushroom_stem_width", routing::get(mushroom_stem_width))
        .route("/api/mushroom_season", routing::get(mushroom_season))
        .with_state(app_state)
        .layer(cors_layer);


    let listener = tokio::net::TcpListener::bind(
        std::net::SocketAddr::from(([127,0,0,1],3000))
    ).await?;

    tracing::info!("Server running on : {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}