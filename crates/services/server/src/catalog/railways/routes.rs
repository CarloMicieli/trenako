use crate::catalog::railways::handlers;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

pub const RAILWAY_ROOT_API: &str = "/api/railways";
pub const RAILWAY_API: &str = "/api/railways/{railwayId}";

pub fn railways_router() -> Router<AppState> {
    Router::new()
        .route(
            RAILWAY_ROOT_API,
            get(handlers::get_all_railways).post(handlers::post_railway),
        )
        .route(
            RAILWAY_API,
            get(handlers::get_railway_by_id)
                .put(handlers::put_railway)
                .delete(handlers::delete_railway),
        )
}
