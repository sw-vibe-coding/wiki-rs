use std::rc::Rc;
use wiki_common::storage::WikiStorage;
use yew_router::prelude::*;

/// Wrapper around a trait object so it can be used as a Yew context.
/// Uses `Rc::ptr_eq` for equality — the context only "changes" if
/// the `Rc` pointer itself changes, which is the correct behavior.
#[derive(Clone)]
pub struct StorageContext(pub Rc<dyn WikiStorage>);

impl PartialEq for StorageContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl std::ops::Deref for StorageContext {
    type Target = dyn WikiStorage;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/wiki/:title")]
    ViewPage { title: String },
    #[at("/wiki/:title/edit")]
    EditPage { title: String },
    #[at("/pages")]
    PageList,
    #[not_found]
    #[at("/404")]
    NotFound,
}
