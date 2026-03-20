mod rest_ops;
mod storage;

use std::rc::Rc;
use storage::RestStorage;
use wiki_common::model::WikiPage;

async fn load_and_render() {
    let storage = Rc::new(RestStorage::new(""));

    if let Some(titles) = fetch_page_list().await {
        let pages = fetch_all_pages(&titles).await;
        storage.populate_cache(pages);
    }

    wiki_ui_app::render_wiki(storage, "Server-backed (REST API)");
}

async fn fetch_page_list() -> Option<Vec<String>> {
    let resp = gloo_net::http::Request::get("/api/pages")
        .send()
        .await
        .ok()?;
    if resp.ok() {
        resp.json::<Vec<String>>().await.ok()
    } else {
        None
    }
}

async fn fetch_all_pages(titles: &[String]) -> Vec<WikiPage> {
    let mut pages = Vec::new();
    for title in titles {
        let url = format!("/api/pages/{title}");
        if let Ok(r) = gloo_net::http::Request::get(&url).send().await
            && let Ok(page) = r.json::<WikiPage>().await
        {
            pages.push(page);
        }
    }
    pages
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    wasm_bindgen_futures::spawn_local(load_and_render());
}
