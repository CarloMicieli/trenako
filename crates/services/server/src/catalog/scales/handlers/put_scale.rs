use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::scale_request::ScaleRequest;

#[tracing::instrument(name = "update_scale", skip(_app_state))]
pub async fn handle(
    Path(_scale_id): Path<ScaleId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<ScaleRequest>,
) -> impl IntoResponse {
    ().into_response()
}
