use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::rolling_stock_id::RollingStockId;

#[tracing::instrument(name = "get_rolling_stock_by_id", skip(_app_state))]
pub async fn handle(
    Path(_catalog_item_id): Path<CatalogItemId>,
    Path(_rolling_stock_id): Path<RollingStockId>,
    State(_app_state): State<AppState>,
) -> impl IntoResponse {
    ().into_response()
}
