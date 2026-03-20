use crate::components::{nav::Nav, page_edit::PageEdit, page_list::PageList, page_view::PageView};
use crate::context::{Route, StorageContext};
use std::rc::Rc;
use wiki_common::storage::WikiStorage;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <Redirect<Route> to={Route::ViewPage { title: "MainPage".to_string() }} />
        },
        Route::ViewPage { title } => html! { <PageView {title} /> },
        Route::EditPage { title } => html! { <PageEdit {title} /> },
        Route::PageList => html! { <PageList /> },
        Route::NotFound => html! { <h1>{"404 -- Page not found"}</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub storage: StorageContext,
    #[prop_or_default]
    pub banner: Option<String>,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <ContextProvider<StorageContext> context={props.storage.clone()}>
            <BrowserRouter>
                if let Some(banner) = &props.banner {
                    <div class="storage-banner">{banner}</div>
                }
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<StorageContext>>
    }
}

/// Convenience function to render the wiki app with a given storage backend.
pub fn render_wiki(storage: Rc<dyn WikiStorage>, banner: &str) {
    let ctx = StorageContext(storage);
    yew::Renderer::<App>::with_props(AppProps {
        storage: ctx,
        banner: Some(banner.to_string()),
    })
    .render();
}
