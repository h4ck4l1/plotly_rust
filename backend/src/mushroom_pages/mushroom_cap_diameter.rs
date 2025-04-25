use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use crate::{AppState, BackendError, CAP_DIAMETER};


pub async fn mushroom_cap_diameter(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse,BackendError> {

    Ok(
        Json(
            app_state.df()
                .column(CAP_DIAMETER)?
                .i16()?
                .into_iter()
                .map(|v| v.unwrap_or(0))
                .collect::<Vec<i16>>()
        )
    )
}