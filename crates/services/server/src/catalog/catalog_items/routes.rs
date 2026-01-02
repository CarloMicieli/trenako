use crate::catalog::catalog_items::handlers;
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub const CATALOG_ITEMS_ROOT_API: &str = "/api/catalog-items";
pub const CATALOG_ITEM_ROOT_API: &str = "/api/catalog-items/{catalogItemId}";
pub const ROLLING_STOCKS_ROOT_API: &str = "/api/catalog-items/{catalogItemId}/rolling-stocks";
pub const ROLLING_STOCK_ROOT_API: &str = "/api/catalog-items/{catalogItemId}/rolling-stocks/{rollingStockId}";

pub fn catalog_items_router() -> Router<AppState> {
    Router::new()
        .route(CATALOG_ITEMS_ROOT_API, post(handlers::post_catalog_item))
        .route(
            CATALOG_ITEM_ROOT_API,
            get(handlers::get_catalog_item_by_id)
                .delete(handlers::delete_catalog_item)
                .put(handlers::put_catalog_item),
        )
        .route(ROLLING_STOCKS_ROOT_API, post(handlers::post_rolling_stock))
        .route(
            ROLLING_STOCK_ROOT_API,
            get(handlers::get_rolling_stock_by_id)
                .delete(handlers::delete_rolling_stock)
                .put(handlers::put_rolling_stock),
        )
}
