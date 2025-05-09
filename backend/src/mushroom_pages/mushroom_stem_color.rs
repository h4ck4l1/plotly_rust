use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, STEM_COLOR};

pub async fn mushroom_stem_color(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let stem_color = app_state.df()
        .column(STEM_COLOR)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let stem_color_json = &ALL_FIT_JSON.clone()[&STEM_COLOR.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "stem_color": stem_color,
            "stem_color_json": stem_color_json,
            "type": "cat"
        })
    ))
}