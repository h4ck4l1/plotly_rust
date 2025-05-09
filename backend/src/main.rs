#![allow(unused)]

use std::sync::Arc;
use axum::{http::HeaderValue, routing, Router};
use backend::{mushroom_pages::{mushroom_cap_diameter::mushroom_cap_diameter, mushroom_cap_shape::mushroom_cap_shape, mushroom_class::mushroom_class, mushroom_gill_attachment::mushroom_gill_attachment, mushroom_gill_color::mushroom_gill_color, mushroom_season::mushroom_season, mushroom_stem_color::mushroom_stem_color, mushroom_stem_height::mushroom_stem_height, mushroom_stem_width::mushroom_stem_width}, AppState, BackendError, ALL_FIT_JSON, MUSHROOM};
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
        .route("/mushroom_cap_shape", routing::get(mushroom_cap_shape))
        .route("/mushroom_gill_attachment", routing::get(mushroom_gill_attachment))
        .route("/mushroom_gill_color", routing::get(mushroom_gill_color))
        .route("/mushroom_class", routing::get(mushroom_class))
        .route("/mushroom_stem_color", routing::get(mushroom_stem_color))
        .route("/mushroom_stem_height", routing::get(mushroom_stem_height))
        .route("/mushroom_stem_width", routing::get(mushroom_stem_width))
        .route("/mushroom_season", routing::get(mushroom_season))
        .with_state(app_state)
        .layer(cors_layer);


    let listener = tokio::net::TcpListener::bind(
        std::net::SocketAddr::from(([127,0,0,1],3000))
    ).await?;

    tracing::info!("Server running on : {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}