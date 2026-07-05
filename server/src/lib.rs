use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use agentready::analyzer::{RepoAnalysis, analyze_repository};
use agentready::harness::{HarnessFilter, HarnessReadinessReport, analyze_harness_readiness};
use agentready::source::{RepositoryTarget, load_snapshot, parse_target};

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/scans", post(create_scan))
        .layer(CorsLayer::permissive())
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
struct ScanRequest {
    target: String,
}

#[derive(Debug, Serialize)]
struct ScanResponse {
    target: String,
    analysis: RepoAnalysis,
    harness: HarnessReadinessReport,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

async fn create_scan(payload: Result<Json<ScanRequest>, JsonRejection>) -> Response {
    let Json(request) = match payload {
        Ok(json) => json,
        Err(rejection) => return error(StatusCode::BAD_REQUEST, rejection.body_text()),
    };

    let target = match parse_target(&request.target) {
        Ok(target) => target,
        Err(parse_error) => return error(StatusCode::BAD_REQUEST, format!("{parse_error:#}")),
    };

    if matches!(target, RepositoryTarget::Local(_)) {
        return error(
            StatusCode::BAD_REQUEST,
            "only GitHub targets are supported: use github:owner/repo or a GitHub URL".to_string(),
        );
    }

    let described = target.describe();
    let snapshot = match tokio::task::spawn_blocking(move || load_snapshot(&target)).await {
        Ok(Ok(snapshot)) => snapshot,
        Ok(Err(load_error)) => {
            return error(upstream_status(&load_error), format!("{load_error:#}"));
        }
        Err(join_error) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, join_error.to_string());
        }
    };

    let response = ScanResponse {
        target: described,
        analysis: analyze_repository(&snapshot),
        harness: analyze_harness_readiness(&snapshot, HarnessFilter::All),
    };

    (StatusCode::OK, Json(response)).into_response()
}

fn upstream_status(load_error: &anyhow::Error) -> StatusCode {
    let text = format!("{load_error:#}");
    if text.contains("404") {
        StatusCode::NOT_FOUND
    } else if text.contains("rate limit") {
        StatusCode::TOO_MANY_REQUESTS
    } else {
        StatusCode::BAD_GATEWAY
    }
}

fn error(status: StatusCode, message: String) -> Response {
    (status, Json(ErrorResponse { error: message })).into_response()
}
