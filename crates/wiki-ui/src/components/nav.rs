use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
            <Link<Route> to={Route::ViewPage { title: "MainPage".to_string() }}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::PageList}>{"All Pages"}</Link<Route>>
        </nav>
    }
}
