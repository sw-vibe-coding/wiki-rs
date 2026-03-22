use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use wiki_common::model::WikiPage;
use wiki_server::api::build_router;
use wiki_server::seed::create_storage;
use wiki_server::storage::BackendKind;

async fn db_router() -> (axum::Router, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::Db, dir.path()).await;
    (build_router(storage), dir)
}

#[tokio::test]
async fn list_pages_returns_seeded() {
    let (app, _dir) = db_router().await;
    let resp = app
        .oneshot(Request::get("/api/pages").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let pages: Vec<String> = serde_json::from_slice(&body).unwrap();
    assert!(pages.contains(&"MainPage".to_string()));
}

#[tokio::test]
async fn crud_lifecycle() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::Db, dir.path()).await;
    let app = build_router(storage);

    // Create
    let page = WikiPage::new("DbTest", "db content", 1000);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/DbTest")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&page).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Read
    let resp = app
        .clone()
        .oneshot(
            Request::get("/api/pages/DbTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let fetched: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched.content, "db content");

    // Head (exists)
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::HEAD)
                .uri("/api/pages/DbTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/pages/DbTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify gone
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::HEAD)
                .uri("/api/pages/DbTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
