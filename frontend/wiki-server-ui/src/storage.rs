use std::cell::RefCell;
use std::collections::HashMap;
use wiki_common::model::WikiPage;

/// REST API-backed storage with local cache.
///
/// Reads return cached data. Writes update cache optimistically
/// and fire async requests to the server in the background.
pub struct RestStorage {
    pub(crate) cache: RefCell<HashMap<String, WikiPage>>,
    pub(crate) base_url: String,
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
