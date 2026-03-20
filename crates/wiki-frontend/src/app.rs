use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{nav::Nav, page_view::PageView, page_edit::PageEdit, page_list::PageList};
use crate::storage::MemoryStorage;

pub type StorageContext = MemoryStorage;

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

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Redirect<Route> to={Route::ViewPage { title: "MainPage".to_string() }} /> },
        Route::ViewPage { title } => html! { <PageView {title} /> },
        Route::EditPage { title } => html! { <PageEdit {title} /> },
        Route::PageList => html! { <PageList /> },
        Route::NotFound => html! { <h1>{"404 — Page not found"}</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let storage = use_memo((), |_| MemoryStorage::new());

    html! {
        <ContextProvider<StorageContext> context={(*storage).clone()}>
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<StorageContext>>
    }
}
