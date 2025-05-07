use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, SEASON};

pub async fn mushroom_season(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let season = app_state.df()
        .column(SEASON)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let season_json = &ALL_FIT_JSON.clone()[&SEASON.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "season": season,
            "season_json": season_json
        })
    ))
}