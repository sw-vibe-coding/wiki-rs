mod storage;
mod toolbar;

use crate::storage::ExportFileStorage;
use crate::toolbar::ExportImportToolbar;
use std::rc::Rc;
use wiki_ui::components::{
    nav::Nav, page_edit::PageEdit, page_list::PageList, page_view::PageView,
};
use wiki_ui::context::{Route, StorageContext};
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

#[function_component(ExportFileApp)]
fn export_file_app() -> Html {
    let storage = use_memo((), |_| {
        StorageContext(
            Rc::new(ExportFileStorage::new()) as Rc<dyn wiki_common::storage::WikiStorage>
        )
    });

    html! {
        <ContextProvider<StorageContext> context={(*storage).clone()}>
            <BrowserRouter>
                <div class="storage-banner">{"Export/Import File (JSON download/upload)"}</div>
                <ExportImportToolbar />
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<StorageContext>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ExportFileApp>::new().render();
}
