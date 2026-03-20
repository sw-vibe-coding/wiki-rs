use crate::storage::RestStorage;
use wiki_common::model::WikiPage;
use wiki_common::storage::WikiStorage;

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
