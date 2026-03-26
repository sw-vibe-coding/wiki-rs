use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct ConflictResponse {
    error: String,
    current_page: Option<WikiPage>,
    current_etag: String,
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
async fn get_returns_etag_header() {
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
    let etag = resp.headers().get("ETag").expect("ETag header missing");
    let etag_str = etag.to_str().unwrap();
    assert!(etag_str.starts_with('"') && etag_str.ends_with('"'));
}

#[tokio::test]
async fn cas_put_succeeds_with_matching_etag() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    let app = build_router(storage);

    // GET to obtain ETag
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

    // PUT with matching If-Match
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

    // Verify content changed
    let resp = app
        .oneshot(
            Request::get("/api/pages/MainPage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let fetched: WikiPage = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched.content, "updated via CAS");
}

#[tokio::test]
async fn cas_put_fails_with_stale_etag() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    let app = build_router(storage);

    // GET to obtain ETag
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

    // Unconditional PUT to change the page (simulating another writer)
    let page = WikiPage::new("MainPage", "changed by someone else", 1000);
    let resp = app
        .clone()
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
    assert_eq!(resp.status(), StatusCode::OK);

    // CAS PUT with stale ETag should fail
    let page2 = WikiPage::new("MainPage", "my conflicting edit", 2000);
    let resp = app
        .clone()
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
    assert!(conflict.current_page.is_some());
    assert_eq!(
        conflict.current_page.unwrap().content,
        "changed by someone else"
    );
    assert!(!conflict.current_etag.is_empty());
}

#[tokio::test]
async fn unconditional_put_still_works() {
    let dir = tempfile::tempdir().unwrap();
    let storage = create_storage(BackendKind::File, dir.path()).await;
    let app = build_router(storage);

    let page = WikiPage::new("TestPage", "no if-match", 1000);
    let resp = app
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
