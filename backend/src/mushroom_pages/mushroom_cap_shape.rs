use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, CAP_SHAPE};


pub async fn mushroom_cap_shape(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {


    let cap_shape = app_state.df()
        .column(CAP_SHAPE)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let cap_shape_json = &ALL_FIT_JSON.clone()[&CAP_SHAPE.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "cap_shape": cap_shape,
            "cap_shape_json": cap_shape_json
        })
    ))
}