use std::fs;

use axum::response::IntoResponse;
use once_cell::sync::Lazy;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use hyper::StatusCode;
use serde_json::Value;

pub mod mushroom_pages;

pub const CAP_DIAMETER: &str = "cap-diameter";
pub const CAP_SHAPE: &str = "cap-shape";
pub const GILL_ATTACHMENT: &str = "gill-attachment";
pub const GILL_COLOR: &str = "gill-color";
pub const STEM_WIDTH: &str = "stem-width";
pub const STEM_HEIGHT: &str = "stem-height";
pub const STEM_COLOR: &str = "stem-color";
pub const SEASON: &str = "season";
pub const CLASS: &str = "class";


pub static MUSHROOM: Lazy<LazyFrame> = Lazy::new(|| {
    LazyCsvReader::new("datafolder/mushroom_cleaned.csv")
        .with_has_header(true)
        .finish()
        .expect("Failed to load the Mushroom LazyFrame")
        
});


pub static ALL_FIT_JSON: Lazy<Value> = Lazy::new(|| {
    let json_string = fs::read_to_string("datafolder/all_cols_fitted_data.json")
        .expect("No ALL_COLS_FITTED_JSON data");

    serde_json::from_str(&json_string)
        .expect("Failed to convert to serde Value")

});


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

#[derive(Debug,thiserror::Error)]
pub enum BackendError {
    #[error("\n Join Error in Task, Unable to Finish the task in thread \nerror: {0}\n")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("\n Polars General Error occurred, Can be from Collecting\nerror: {0}\n")]
    PolarsGeneralError(#[from] polars::error::PolarsError),
    #[error("\n General Std Io Error\nerror: {0}\n")]
    StdIoError(#[from] std::io::Error)
}


impl IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        let (status_code,error_string) = match self {
            BackendError::PolarsGeneralError(error) => (StatusCode::FAILED_DEPENDENCY,error.to_string()),
            BackendError::TokioJoinError(error) => (StatusCode::EXPECTATION_FAILED,error.to_string()),
            BackendError::StdIoError(error) => (StatusCode::EXPECTATION_FAILED,error.to_string()),
        };

        (status_code,error_string).into_response()
    }
}
