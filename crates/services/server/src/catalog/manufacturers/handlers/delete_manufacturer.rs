use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::manufacturers::manufacturer_id::ManufacturerId;

#[tracing::instrument(name = "delete_manufacturer", skip(_app_state))]
pub async fn handle(
    Path(_manufacturer_id): Path<ManufacturerId>,
    State(_app_state): State<AppState>,
) -> impl IntoResponse {
    ().into_response()
}
