#![allow(unused)]

mod mushroom_pages;

use anyhow::Result;
use axum::{http::HeaderValue, response::IntoResponse, routing, Router};
use hyper::{Method, StatusCode};
use mushroom_pages::mushroom_cap_diameter::mushroom_cap_diameter;
use once_cell::sync::Lazy;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

pub static MUSHROOM: Lazy<LazyFrame> = Lazy::new(|| {
    LazyCsvReader::new("datafolder/mushroom_cleaned.csv")
        .with_has_header(true)
        .finish()
        .expect("Failed to load the Mushroom LazyFrame")
        
});

pub const CAP_DIAMETER: &str = "cap-diameter";
pub const CAP_SHAPE: &str = "cap-shape";
pub const GILL_ATTACHMENT: &str = "gill-attachment";
pub const GILL_COLOR: &str = "gill-color";
pub const STEM_WIDTH: &str = "stem-width";
pub const STEM_HEIGHT: &str = "stem-height";
pub const STEM_COLOR: &str = "stem-color";
pub const SEASON: &str = "season";
pub const CLASS: &str = "class";


#[tokio::main]
async fn main() -> Result<()> {
    
    tracing_subscriber::fmt::init();


    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(["http://localhost:8080".parse::<HeaderValue>().unwrap()])
        .allow_headers(Any);

    let app_state = Arc::new(AppState::new(

        // Apply Appwide Dataframe transformations here
        
        MUSHROOM
            .to_owned()
            .select([all().shrink_dtype()])
            .collect()?
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

#[derive(Debug,Serialize,Deserialize)]
pub struct AppState {
    df: DataFrame
}

impl AppState {
    pub fn new(df: DataFrame) -> Self {
        Self { df }
    }

    pub fn df(&self) -> &DataFrame {
        &self.df
    }
}

#[derive(Debug)]
pub struct BackendError {
    error: anyhow::Error
}


impl IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST,self.error.to_string()).into_response()
    }
}


impl From<tokio::task::JoinError> for BackendError {
    fn from(value: tokio::task::JoinError) -> Self {
        BackendError { error: anyhow::Error::from(value) }
    }
}

impl From<polars::error::PolarsError> for BackendError {
    fn from(value: polars::error::PolarsError) -> Self {
        BackendError { error: anyhow::Error::from(value) }
    }
}