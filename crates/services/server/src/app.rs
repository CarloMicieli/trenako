use crate::catalog::catalog_router;
use crate::health_check;
use crate::state::AppState;
use axum;
use axum::Router;
use axum::routing::get;
use axum_prometheus::PrometheusMetricLayer;
use axum_prometheus::metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use configuration::Settings;
use hyper::http::HeaderName;
use std::sync::OnceLock;
use tokio::net::TcpListener;
use tower_http::LatencyUnit;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

static METRIC_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Run the web server
pub async fn run(tcp_listener: TcpListener, settings: &Settings) {
    axum::serve(tcp_listener, build_app(settings)).await.unwrap();
}

pub fn build_app(settings: &Settings) -> Router {
    let app_state = AppState::from_settings(settings);
    let metric_handle = METRIC_HANDLE
        .get_or_init(|| {
            PrometheusBuilder::new()
                .install_recorder()
                .expect("failed to install Prometheus recorder; a global recorder may already be installed")
        })
        .clone();
    let prometheus_layer = PrometheusMetricLayer::new();
    let management_router = Router::new().route("/health-check", get(health_check::handler)).route(
        "/metrics",
        get(move || {
            let metric_handle = metric_handle.clone();
            async move {
                metric_handle.run_upkeep();
                metric_handle.render()
            }
        }),
    );

    let x_request_id = HeaderName::from_static("x-request-id");

    catalog_router()
        .merge(management_router)
        .with_state(app_state.clone())
        .layer(prometheus_layer)
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
