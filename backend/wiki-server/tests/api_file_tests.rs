use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use wiki_common::model::WikiPage;
use wiki_server::api::build_router;
use wiki_server::seed::create_storage;
use wiki_server::storage::BackendKind;

async fn file_router() -> (axum::Router, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    (build_router(storage), dir)
}

#[tokio::test]
async fn list_pages_returns_seeded() {
    let (app, _dir) = file_router().await;
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
async fn get_seeded_page() {
    let (app, _dir) = file_router().await;
    let resp = app
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let page: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(page.title, "MainPage");
}

#[tokio::test]
async fn get_missing_page_returns_404() {
    let (app, _dir) = file_router().await;
    let resp = app
        .oneshot(
            Request::get("/api/pages/NoSuchPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn put_then_get_page() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    let app = build_router(storage);
    let page = WikiPage::new("TestPage", "hello world", 1000);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/pages/TestPage")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&page).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app
        .oneshot(
            Request::get("/api/pages/TestPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let fetched: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched.content, "hello world");
}

#[tokio::test]
async fn delete_page_removes_it() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    let app = build_router(storage);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
