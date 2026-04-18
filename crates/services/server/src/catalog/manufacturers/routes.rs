use crate::catalog::manufacturers::handlers;
use crate::state::AppState;
use axum::Router;
use axum::routing::{delete, get};

pub const MANUFACTURERS_ROOT_API: &str = "/api/manufacturers";
pub const MANUFACTURER_ROOT_API: &str = "/api/manufacturers/{manufacturer_id}";

pub fn manufacturers_router() -> Router<AppState> {
    Router::new()
        .route(
            MANUFACTURERS_ROOT_API,
            get(handlers::get_all_manufacturers).post(handlers::post_manufacturer),
        )
        .route(
            MANUFACTURER_ROOT_API,
            delete(handlers::delete_manufacturer)
                .get(handlers::get_manufacturer_by_id)
                .put(handlers::put_manufacturer),
        )
}
