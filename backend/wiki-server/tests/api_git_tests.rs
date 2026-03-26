use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde::Deserialize;
use tower::ServiceExt;
use wiki_common::model::WikiPage;
use wiki_server::api::build_router;
use wiki_server::seed::create_storage;
use wiki_server::storage::BackendKind;

async fn git_router() -> (axum::Router, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::Git, dir.path()).await;
    (build_router(storage), dir)
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ConflictResponse {
    error: String,
    current_page: Option<WikiPage>,
    current_etag: String,
}

#[tokio::test]
async fn list_pages_returns_seeded() {
    let (app, _dir) = git_router().await;
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
    let storage = create_storage(BackendKind::Git, dir.path()).await;
    let app = build_router(storage);

    // Create
    let page = WikiPage::new("GitTest", "git content", 1000);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/GitTest")
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
            Request::get("/api/pages/GitTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let fetched: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched.content, "git content");

    // Update
    let updated = WikiPage::new("GitTest", "updated content", 2000);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/GitTest")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&updated).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app
        .clone()
        .oneshot(
            Request::get("/api/pages/GitTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let fetched: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched.content, "updated content");

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/pages/GitTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app
        .oneshot(
            Request::get("/api/pages/GitTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_returns_etag_header() {
    let (app, _dir) = git_router().await;
    let resp = app
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let etag = resp.headers().get("ETag").expect("ETag header missing");
    let etag_str = etag.to_str().unwrap();
    assert!(etag_str.starts_with('"') && etag_str.ends_with('"'));
}

#[tokio::test]
async fn cas_put_succeeds_with_matching_etag() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::Git, dir.path()).await;
    let app = build_router(storage);

    let resp = app
        .clone()
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let etag = resp
        .headers()
        .get("ETag")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let page = WikiPage::new("MainPage", "updated via CAS", 1000);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/MainPage")
                .header("content-type", "application/json")
                .header("If-Match", &etag)
                .body(Body::from(serde_json::to_vec(&page).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp.headers().get("ETag").is_some());
}

#[tokio::test]
async fn cas_put_fails_with_stale_etag() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::Git, dir.path()).await;
    let app = build_router(storage);

    let resp = app
        .clone()
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let old_etag = resp
        .headers()
        .get("ETag")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let page = WikiPage::new("MainPage", "changed by someone else", 1000);
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/MainPage")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&page).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let page2 = WikiPage::new("MainPage", "conflicting edit", 2000);
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/MainPage")
                .header("content-type", "application/json")
                .header("If-Match", &old_etag)
                .body(Body::from(serde_json::to_vec(&page2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let conflict: ConflictResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(conflict.error, "conflict");
    assert_eq!(
        conflict.current_page.unwrap().content,
        "changed by someone else"
    );
}
