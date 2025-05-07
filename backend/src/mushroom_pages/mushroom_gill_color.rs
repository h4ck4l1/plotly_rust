use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, GILL_COLOR};

pub async fn mushroom_gill_color(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let gill_color = app_state.df()
        .column(GILL_COLOR)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let gill_color_json = &ALL_FIT_JSON.clone()[&GILL_COLOR.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "gill_color": gill_color,
            "gill_color_json": gill_color_json
        })
    ))
}