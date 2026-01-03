use crate::catalog::catalog_router;
use crate::health_check;
use crate::state::AppState;
use axum;
use axum::Router;
use axum::routing::get;
use configuration::Settings;
use hyper::http::HeaderName;
use tokio::net::TcpListener;
use tower_http::LatencyUnit;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

/// Run the web server
pub async fn run(tcp_listener: TcpListener, settings: &Settings) {
    axum::serve(tcp_listener, build_app(settings)).await.unwrap();
}

pub fn build_app(settings: &Settings) -> Router {
    let app_state = AppState::from_settings(settings);
    let management_router = Router::new().route("/health-check", get(health_check::handler));

    let x_request_id = HeaderName::from_static("x-request-id");

    catalog_router()
        .merge(management_router)
        .with_state(app_state.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true).level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(x_request_id))
        .layer(CompressionLayer::new())
}
