//! Static content routes and metrics endpoint.
//!
//! Provides health checks, version info, Stripe config, and Prometheus metrics.

use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;
use crate::web::handlers::static_content::{health_handler, version_handler, stripe_config_handler};

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
            .unwrap()
            .install_recorder()
            .unwrap()
    });

    Router::new()
        .route("/health", get(health_handler))
        .route("/status/version", get(version_handler))
        .route("/api/stripe-config", get(stripe_config_handler))
        .route(
            "/metrics",
            get(move || async move { recorder_handle.render() }),
        )
}
