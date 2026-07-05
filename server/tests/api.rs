use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use tower::ServiceExt;

use agentready_server::{AppState, app};

async fn body_text(response: axum::response::Response) -> String {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn scan_request(json: &str) -> Request<Body> {
    Request::post("/api/scans")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.to_string()))
        .expect("request")
}

#[tokio::test]
async fn health_returns_ok() {
    let response = app(AppState::default())
        .oneshot(
            Request::get("/health")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(body_text(response).await, "ok");
}

#[tokio::test]
async fn scan_rejects_local_targets() {
    let response = app(AppState::default())
        .oneshot(scan_request(r#"{"target": "."}"#))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(body_text(response).await.contains("GitHub"));
}

#[tokio::test]
async fn scan_rejects_malformed_github_targets() {
    let response = app(AppState::default())
        .oneshot(scan_request(r#"{"target": "github:owner-without-repo"}"#))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(
        body_text(response)
            .await
            .contains("expected github:owner/repo")
    );
}

#[tokio::test]
async fn scan_rejects_invalid_json_bodies() {
    let response = app(AppState::default())
        .oneshot(scan_request(r#"{"nope": true}"#))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[ignore = "requires network access; run manually with cargo test -p agentready-server -- --ignored"]
async fn scan_returns_readiness_report_live() {
    let response = app(AppState::default())
        .oneshot(scan_request(
            r#"{"target": "github:stephanemalho/agentready"}"#,
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("\"score\""));
    assert!(body.contains("github:stephanemalho/agentready"));
}
