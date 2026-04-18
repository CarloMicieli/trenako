//! the module includes everything related to catalog web handlers

use crate::catalog::catalog_items::routes::catalog_items_router;
use crate::catalog::manufacturers::routes::manufacturers_router;
use crate::catalog::railways::routes::railways_router;
use crate::catalog::scales::routes::scales_router;
use crate::state::AppState;
use axum::Router;

pub mod catalog_items;
pub mod manufacturers;
pub mod railways;
pub mod scales;

pub fn catalog_router() -> Router<AppState> {
    manufacturers_router()
        .merge(catalog_items_router())
        .merge(railways_router())
        .merge(scales_router())
}
