use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, head, patch, put};
use axum::{Json, Router};
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use wiki_common::async_storage::AsyncWikiStorage;
use wiki_common::etag::content_etag;
use wiki_common::model::WikiPage;
use wiki_common::patch::PatchRequest;

#[derive(Clone)]
struct AppState {
    storage: Arc<dyn AsyncWikiStorage>,
    page_locks: Arc<DashMap<String, Arc<Mutex<()>>>>,
}

pub fn build_router(storage: Arc<dyn AsyncWikiStorage>) -> Router {
    let state = AppState {
        storage,
        page_locks: Arc::new(DashMap::new()),
    };
    Router::new()
        .route("/api/pages", get(list_pages))
        .route("/api/pages/{title}", get(get_page))
        .route("/api/pages/{title}", put(save_page))
        .route("/api/pages/{title}", patch(patch_page))
        .route("/api/pages/{title}", delete(delete_page))
        .route("/api/pages/{title}", head(has_page))
        .with_state(state)
}

async fn list_pages(State(s): State<AppState>) -> Json<Vec<String>> {
    Json(s.storage.list_pages().await)
}

async fn get_page(
    State(s): State<AppState>,
    Path(title): Path<String>,
) -> Result<Response, StatusCode> {
    let page = s
        .storage
        .get_page(&title)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;
    let etag = content_etag(&page.content);
    let body = serde_json::to_vec(&page).unwrap();
    Ok(Response::builder()
        .header("ETag", &etag)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap())
}

async fn save_page(
    State(s): State<AppState>,
    Path(title): Path<String>,
    headers: HeaderMap,
    Json(mut page): Json<WikiPage>,
) -> Response {
    page.title = title.clone();

    let if_match = headers
        .get("If-Match")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match if_match {
        None => {
            // Unconditional write (backward compatible)
            s.storage.save_page(page).await;
            StatusCode::OK.into_response()
        }
        Some(expected_etag) => {
            // CAS: acquire per-page lock
            let lock = s
                .page_locks
                .entry(title.clone())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone();
            let _guard = lock.lock().await;

            let current = s.storage.get_page(&title).await;
            let current_etag = current
                .as_ref()
                .map(|p| content_etag(&p.content))
                .unwrap_or_else(|| "\"\"".to_string());

            if current_etag == expected_etag {
                let new_etag = content_etag(&page.content);
                s.storage.save_page(page).await;
                Response::builder()
                    .status(StatusCode::OK)
                    .header("ETag", &new_etag)
                    .body(Body::empty())
                    .unwrap()
            } else {
                let conflict = ConflictResponse {
                    error: "conflict".to_string(),
                    message: "Page was modified by another writer".to_string(),
                    current_page: current,
                    current_etag: current_etag.clone(),
                };
                Response::builder()
                    .status(StatusCode::CONFLICT)
                    .header("ETag", &current_etag)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_vec(&conflict).unwrap()))
                    .unwrap()
            }
        }
    }
}

async fn patch_page(
    State(s): State<AppState>,
    Path(title): Path<String>,
    Json(req): Json<PatchRequest>,
) -> Response {
    let lock = s
        .page_locks
        .entry(title.clone())
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .clone();
    let _guard = lock.lock().await;

    let Some(page) = s.storage.get_page(&title).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let current_etag = content_etag(&page.content);
    // Accept ETag with or without HTTP quotes: "abc..." or abc...
    let req_etag = if req.etag.starts_with('"') {
        req.etag.clone()
    } else {
        format!("\"{}\"", req.etag)
    };
    if current_etag != req_etag {
        let conflict = ConflictResponse {
            error: "conflict".to_string(),
            message: "Page was modified by another writer".to_string(),
            current_page: Some(page),
            current_etag: current_etag.clone(),
        };
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .header("ETag", &current_etag)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&conflict).unwrap()))
            .unwrap();
    }

    match wiki_common::patch::apply_ops(&page.content, &req.ops) {
        Ok(patched) => {
            let new_etag = content_etag(&patched);
            let mut updated = page;
            updated.content = patched;
            updated.updated_at = wiki_common::time::now();
            s.storage.save_page(updated).await;
            Response::builder()
                .status(StatusCode::OK)
                .header("ETag", &new_etag)
                .body(Body::empty())
                .unwrap()
        }
        Err(e) => {
            let resp = MatchNotFoundResponse {
                error: "match_not_found".to_string(),
                message: format!("Op {} match string not found in page content", e.op_index),
                op_index: e.op_index,
                match_str: e.match_str,
                current_page: Some(page),
                current_etag,
            };
            Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&resp).unwrap()))
                .unwrap()
        }
    }
}

async fn delete_page(State(s): State<AppState>, Path(title): Path<String>) -> StatusCode {
    s.storage.delete_page(&title).await;
    StatusCode::OK
}

async fn has_page(State(s): State<AppState>, Path(title): Path<String>) -> Response {
    match s.storage.get_page(&title).await {
        Some(page) => Response::builder()
            .status(StatusCode::OK)
            .header("ETag", content_etag(&page.content))
            .body(Body::empty())
            .unwrap(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Serialize)]
struct ConflictResponse {
    error: String,
    message: String,
    current_page: Option<WikiPage>,
    current_etag: String,
}

#[derive(Serialize)]
struct MatchNotFoundResponse {
    error: String,
    message: String,
    op_index: usize,
    #[serde(rename = "match")]
    match_str: String,
    current_page: Option<WikiPage>,
    current_etag: String,
}
