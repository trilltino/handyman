//! Static content routes and metrics endpoint.
//!
//! Provides version info and Prometheus metrics.

use crate::web::handlers::static_content::version_handler;
use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static METRICS_RECORDER: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn routes() -> Router {
    // Initialize metrics recorder
    let recorder_handle = METRICS_RECORDER.get_or_init(|| {
        PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Prefix("http_requests_duration_seconds".to_string()),
                &[
                    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ],
            )
            .expect("Failed to set prometheus buckets")
            .install_recorder()
            .expect("Failed to install prometheus recorder")
    });

    Router::new()
        .route("/status/version", get(version_handler))
        .route(
            "/metrics",
            get(move || async move { recorder_handle.render() }),
        )
}
