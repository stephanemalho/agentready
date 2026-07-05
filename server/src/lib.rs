use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use agentready::analyzer::{RepoAnalysis, analyze_repository};
use agentready::harness::{HarnessFilter, HarnessReadinessReport, analyze_harness_readiness};
use agentready::source::{RepositoryTarget, load_snapshot, parse_target};

pub mod store;

#[derive(Clone, Default)]
pub struct AppState {
    pub pool: Option<PgPool>,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/scans", post(create_scan))
        .route("/api/scans/{id}", get(get_scan))
        .route("/api/repositories/{owner}/{repo}/scans", get(list_scans))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn state_from_env() -> anyhow::Result<AppState> {
    let pool = match std::env::var("DATABASE_URL") {
        Ok(database_url) => Some(store::connect(&database_url).await?),
        Err(_) => {
            eprintln!("DATABASE_URL is not set: running without scan history");
            None
        }
    };

    Ok(AppState { pool })
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
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_id: Option<i64>,
    analysis: RepoAnalysis,
    harness: HarnessReadinessReport,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

async fn create_scan(
    State(state): State<AppState>,
    payload: Result<Json<ScanRequest>, JsonRejection>,
) -> Response {
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

    let analysis = analyze_repository(&snapshot);
    let harness = analyze_harness_readiness(&snapshot, HarnessFilter::All);

    let scan_id = match &state.pool {
        Some(pool) => match store::persist_scan(pool, &analysis, &harness).await {
            Ok(scan_id) => Some(scan_id),
            Err(store_error) => {
                return error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("scan succeeded but storing it failed: {store_error:#}"),
                );
            }
        },
        None => None,
    };

    let response = ScanResponse {
        target: described,
        scan_id,
        analysis,
        harness,
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn get_scan(State(state): State<AppState>, Path(id): Path<i64>) -> Response {
    let Some(pool) = &state.pool else {
        return history_unavailable();
    };

    match store::get_scan(pool, id).await {
        Ok(Some(scan)) => (StatusCode::OK, Json(scan)).into_response(),
        Ok(None) => error(StatusCode::NOT_FOUND, format!("no scan with id {id}")),
        Err(store_error) => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{store_error:#}"),
        ),
    }
}

async fn list_scans(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Response {
    let Some(pool) = &state.pool else {
        return history_unavailable();
    };

    match store::list_scans(pool, &owner, &repo).await {
        Ok(scans) => (StatusCode::OK, Json(scans)).into_response(),
        Err(store_error) => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{store_error:#}"),
        ),
    }
}

fn history_unavailable() -> Response {
    error(
        StatusCode::SERVICE_UNAVAILABLE,
        "scan history is unavailable: the server runs without a database".to_string(),
    )
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
