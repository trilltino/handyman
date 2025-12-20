use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use lib_core::model::ModelManager;
use serde_json::json;
use sqlx::query;
use tracing::error;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/health", get(api_health_handler))
        .with_state(mm)
}

async fn api_health_handler(
    State(mm): State<ModelManager>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Check Database Connection
    let db_status = match query("SELECT 1").execute(mm.dbx().db()).await {
        Ok(_) => "ok",
        Err(e) => {
            error!("Health check database query failed: {}", e);
            "error_query"
        }
    };

    // Determine overall status
    let status = if db_status == "ok" {
        "healthy"
    } else {
        "degraded"
    };
    let http_status = if db_status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        http_status,
        Json(json!({
            "status": status,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),
            "checks": {
                "database": db_status
            }
        })),
    )
}
