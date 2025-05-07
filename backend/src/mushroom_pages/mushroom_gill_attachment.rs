use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, ALL_FIT_JSON, GILL_ATTACHMENT};

pub async fn mushroom_gill_attachment(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    let gill_attachment = app_state.df()
        .column(GILL_ATTACHMENT)?
        .f32()?
        .to_vec()
        .into_iter()
        .map(|f| f.unwrap_or(0f32))
        .collect::<Vec<f32>>();

    let gill_attachment_json = &ALL_FIT_JSON.clone()[&GILL_ATTACHMENT.replace("-", "_")];
    Ok(Json(
        serde_json::json!({
            "gill_attachment": gill_attachment,
            "gill_attachment_json": gill_attachment_json
        })
    ))
}