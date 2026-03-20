use crate::context::{Route, StorageContext};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PageList)]
pub fn page_list() -> Html {
    let storage = use_context::<StorageContext>().unwrap();
    let pages = storage.list_pages();

    html! {
        <div>
            <h1 class="page-title">{"All Pages"}</h1>
            <div class="page-list">
                {for pages.iter().map(|title| html! {
                    <Link<Route> to={Route::ViewPage { title: title.clone() }}>{title}</Link<Route>>
                })}
            </div>
        </div>
    }
}
