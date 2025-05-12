#![allow(unused)]

use std::sync::Arc;
use axum::{extract::{Query, State}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smartcore::{linalg::basic::matrix::DenseMatrix, linear::linear_regression::{LinearRegression, LinearRegressionParameters}};
use crate::{AppState, BackendError, CONT_COLS};

pub async fn mushroom_handler(
    State(app_state): State<Arc<AppState>>,
    Query(query_params): Query<MushroomQuery>
) -> Result<impl IntoResponse, BackendError> {
    
    let col_name = query_params.col_name;
    let col_data = app_state.df()
        .column(&col_name.replace("_", "-"))?
        .f32()?
        .to_vec()
        .iter().map(|v| v.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    
    let mut mushroom_response = MushroomResponse::new(col_data.clone());

    if let Some(s) = query_params.fit_col_name {
        let y = app_state.df()
            .column(&s.replace("_", "-"))?
            .f32()?
            .iter().map(|f| f.unwrap_or(0f32))
            .collect::<Vec<f32>>();
        let x_dense = DenseMatrix::new(col_data.len(), 1, col_data.clone(), true)?;
        let lr = LinearRegression::fit(&x_dense, &y, LinearRegressionParameters::default().with_solver(smartcore::linear::linear_regression::LinearRegressionSolverName::QR))?;
        let y_hat = lr.predict(&x_dense)?;
        mushroom_response = mushroom_response.with_fit_data(y_hat)
            .with_second_col_data(y);
    } else {
        mushroom_response = mushroom_response.with_col_json(
            (&app_state.json()[&col_name]).to_owned()
        )
        .with_type_of_col(
            if (*CONT_COLS).contains(&col_name.as_str()) {
                "cont"
            } else {
                "cat"
            }
        );
    };    

    Ok(Json(mushroom_response))
}


#[derive(Debug,Deserialize)]
pub struct MushroomQuery {
    col_name: String,
    fit_col_name: Option<String>
}


#[derive(Debug,Serialize,Default)]
pub struct MushroomResponse {
    col_data: Vec<f32>,
    #[serde(skip_serializing_if="Option::is_none")]
    col_json: Option<Value>,
    #[serde(skip_serializing_if="Option::is_none")]
    type_of_col: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    second_col_data: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    fit_data: Option<Vec<f32>>
}

impl MushroomResponse {
    pub fn new(col_data: Vec<f32>) -> Self {
        Self { col_data, col_json: None, type_of_col: None, second_col_data: None ,fit_data: None }
    }

    pub fn with_col_json(mut self, col_json: Value) ->  Self {
        self.col_json = Some(col_json);
        self
    }

    pub fn with_type_of_col(mut self, type_of_col: &str) -> Self {
        self.type_of_col = Some(type_of_col.to_string());
        self
    }

    pub fn with_second_col_data(mut self, second_col_data: Vec<f32>) -> Self {
        self.second_col_data = Some(second_col_data);
        self
    }

    pub fn with_fit_data(mut self, fit_data: Vec<f32>) -> Self {
        self.fit_data = Some(fit_data);
        self
    }

    pub fn finish(&self) -> &Self {
        self
    }

}