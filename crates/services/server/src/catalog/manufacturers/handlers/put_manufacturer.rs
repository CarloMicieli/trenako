use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::manufacturers::manufacturer_id::ManufacturerId;
use catalog::manufacturers::manufacturer_request::ManufacturerRequest;

#[tracing::instrument(name = "update_manufacturer", skip(_app_state))]
pub async fn handle(
    Path(_manufacturer_id): Path<ManufacturerId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<ManufacturerRequest>,
) -> impl IntoResponse {
    ().into_response()
}
