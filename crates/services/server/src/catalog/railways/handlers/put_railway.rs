use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::manufacturers::manufacturer_request::ManufacturerRequest;
use catalog::railways::railway_id::RailwayId;

#[tracing::instrument(name = "update_railway", skip(_app_state))]
pub async fn handle(
    Path(_railway_id): Path<RailwayId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<ManufacturerRequest>,
) -> impl IntoResponse {
    ().into_response()
}
