use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;

#[tracing::instrument(name = "update_rolling_stock", skip(_app_state))]
pub async fn handle(
    Path(_catalog_item_id): Path<CatalogItemId>,
    Path(_rolling_stock_id): Path<RollingStockId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<RollingStockRequest>,
) -> impl IntoResponse {
    ().into_response()
}
