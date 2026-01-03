use crate::catalog::scales::handlers;
use crate::state::AppState;
use axum::Router;
use axum::routing::{delete, get};

pub const SCALE_ROOT_API: &str = "/api/scales";
pub const SCALE_API: &str = "/api/scales/{scale_id}";

pub fn scales_router() -> Router<AppState> {
    Router::new()
        .route(SCALE_ROOT_API, get(handlers::get_all_scales).post(handlers::post_scale))
        .route(
            SCALE_API,
            delete(handlers::delete_scale)
                .get(handlers::get_scale_by_id)
                .put(handlers::put_scale),
        )
}
