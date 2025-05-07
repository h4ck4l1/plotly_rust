use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, STEM_HEIGHT};

pub async fn mushroom_stem_height(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let stem_height = app_state.df()
        .column(STEM_HEIGHT)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let stem_heigth_json = &ALL_FIT_JSON.clone()[&STEM_HEIGHT.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "stem_height": stem_height,
            "stem_heigth_json": stem_heigth_json
        })
    ))
}