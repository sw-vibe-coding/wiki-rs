use crate::context::{Route, StorageContext};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use wiki_common::parser::render_wiki_content;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(PageView)]
pub fn page_view(props: &Props) -> Html {
    let storage = use_context::<StorageContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let title = props.title.clone();

    let page = storage.get_page(&title);

    let content_ref = use_node_ref();

    // Handle clicks on wiki links inside rendered content
    {
        let content_ref = content_ref.clone();
        let navigator = navigator.clone();
        let storage = storage.clone();
        use_effect_with(title.clone(), move |_| {
            if let Some(element) = content_ref.cast::<HtmlElement>() {
                let links = element.query_selector_all("a.wiki-link").unwrap();
                for i in 0..links.length() {
                    let link = links.get(i).unwrap();
                    let link: HtmlElement = link.unchecked_into();

                    // Red-link styling for missing pages
                    if let Some(target_title) = link.get_attribute("data-wiki-link").as_deref()
                        && !storage.has_page(target_title)
                    {
                        let _ = link.class_list().add_1("wiki-link-missing");
                    }

                    let nav = navigator.clone();
                    let stor = storage.clone();
                    let closure = gloo::events::EventListener::new(&link, "click", move |e| {
                        e.prevent_default();
                        let target: HtmlElement = e.target().unwrap().unchecked_into();
                        if let Some(wiki_target) = target.get_attribute("data-wiki-link").as_deref()
                        {
                            if stor.has_page(wiki_target) {
                                nav.push(&Route::ViewPage {
                                    title: wiki_target.to_string(),
                                });
                            } else {
                                nav.push(&Route::EditPage {
                                    title: wiki_target.to_string(),
                                });
                            }
                        }
                    });
                    closure.forget();
                }
            }
        });
    }

    let on_delete = {
        let title = title.clone();
        let storage = storage.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            storage.delete_page(&title);
            navigator.push(&Route::ViewPage {
                title: "MainPage".to_string(),
            });
        })
    };

    match page {
        Some(page) => {
            let rendered = render_wiki_content(&page.content);
            let is_main = page.title == "MainPage";
            html! {
                <div>
                    <h1 class="page-title">{&page.title}</h1>
                    <div ref={content_ref}>
                        {Html::from_html_unchecked(AttrValue::from(rendered))}
                    </div>
                    <hr/>
                    <Link<Route> to={Route::EditPage { title: page.title.clone() }}>{"Edit this page"}</Link<Route>>
                    if !is_main {
                        {" | "}<button onclick={on_delete}>{"Delete this page"}</button>
                    }
                </div>
            }
        }
        None => {
            html! {
                <div>
                    <h1 class="page-title">{&title}</h1>
                    <p>{"This page does not exist yet."}</p>
                    <Link<Route> to={Route::EditPage { title: title.clone() }}>{"Create this page"}</Link<Route>>
                </div>
            }
        }
    }
}
