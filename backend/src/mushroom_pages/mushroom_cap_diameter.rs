use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, CAP_DIAMETER};


pub async fn mushroom_cap_diameter(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {


    let cap_diameter = app_state.df()
        .column(CAP_DIAMETER)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let cap_diameter_json = &ALL_FIT_JSON.clone()[&CAP_DIAMETER.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "cap_diameter": cap_diameter,
            "cap_diameter_json": cap_diameter_json
        })
    ))

}