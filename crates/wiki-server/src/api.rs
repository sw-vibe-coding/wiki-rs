use crate::storage::AsyncWikiStorage;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, head, put};
use axum::{Json, Router};
use std::sync::Arc;
use wiki_common::model::WikiPage;

type Storage = Arc<dyn AsyncWikiStorage>;

pub fn build_router(storage: Storage) -> Router {
    Router::new()
        .route("/api/pages", get(list_pages))
        .route("/api/pages/{title}", get(get_page))
        .route("/api/pages/{title}", put(save_page))
        .route("/api/pages/{title}", delete(delete_page))
        .route("/api/pages/{title}", head(has_page))
        .with_state(storage)
}

async fn list_pages(State(s): State<Storage>) -> Json<Vec<String>> {
    Json(s.list_pages().await)
}

async fn get_page(
    State(s): State<Storage>,
    Path(title): Path<String>,
) -> Result<Json<WikiPage>, StatusCode> {
    s.get_page(&title)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn save_page(
    State(s): State<Storage>,
    Path(title): Path<String>,
    Json(mut page): Json<WikiPage>,
) -> StatusCode {
    page.title = title;
    s.save_page(page).await;
    StatusCode::OK
}

async fn delete_page(State(s): State<Storage>, Path(title): Path<String>) -> StatusCode {
    s.delete_page(&title).await;
    StatusCode::OK
}

async fn has_page(State(s): State<Storage>, Path(title): Path<String>) -> StatusCode {
    if s.has_page(&title).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
