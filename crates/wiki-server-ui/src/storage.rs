use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

/// REST API-backed storage with local cache.
///
/// Reads return cached data. Writes update cache optimistically
/// and fire async requests to the server in the background.
pub struct RestStorage {
    cache: RefCell<HashMap<String, WikiPage>>,
    base_url: String,
}

impl RestStorage {
    pub fn new(base_url: &str) -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
            base_url: base_url.to_string(),
        }
    }

    pub fn populate_cache(&self, pages: Vec<WikiPage>) {
        let mut cache = self.cache.borrow_mut();
        for page in pages {
            cache.insert(page.title.clone(), page);
        }
    }
}

impl WikiStorage for RestStorage {
    fn get_page(&self, title: &str) -> Option<WikiPage> {
        self.cache.borrow().get(title).cloned()
    }

    fn save_page(&self, page: WikiPage) {
        self.cache
            .borrow_mut()
            .insert(page.title.clone(), page.clone());
        let url = format!("{}/api/pages/{}", self.base_url, page.title);
        let body = serde_json::to_string(&page).unwrap_or_default();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = gloo_net::http::Request::put(&url)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap()
                .send()
                .await;
        });
    }

    fn delete_page(&self, title: &str) {
        self.cache.borrow_mut().remove(title);
        let url = format!("{}/api/pages/{title}", self.base_url);
        wasm_bindgen_futures::spawn_local(async move {
            let _ = gloo_net::http::Request::delete(&url).send().await;
        });
    }

    fn list_pages(&self) -> Vec<String> {
        let mut titles: Vec<String> = self.cache.borrow().keys().cloned().collect();
        titles.sort();
        titles
    }

    fn has_page(&self, title: &str) -> bool {
        self.cache.borrow().contains_key(title)
    }
}
