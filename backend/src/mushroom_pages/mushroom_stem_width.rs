use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, STEM_WIDTH};

pub async fn mushroom_stem_width(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let stem_width = app_state.df()
        .column(STEM_WIDTH)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let stem_width_json = &ALL_FIT_JSON.clone()[&STEM_WIDTH.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "stem_width": stem_width,
            "stem_width_json": stem_width_json
        })
    ))
}