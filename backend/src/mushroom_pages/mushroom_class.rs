use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, CLASS};

pub async fn mushroom_class(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let class = app_state.df()
        .column(CLASS)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let class_json = &ALL_FIT_JSON.clone()[&CLASS.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "class": class,
            "class_json": class_json,
            "type": "cat"
        })
    ))
}