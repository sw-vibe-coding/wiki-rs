use crate::{Route, StorageContext};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use wiki_common::aging;
use wiki_common::parser::render_wiki_content;
use wiki_common::time;
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

    setup_wiki_links(
        content_ref.clone(),
        navigator.clone(),
        storage.clone(),
        &title,
    );

    let on_delete = build_delete_callback(title.clone(), storage, navigator);

    render_page(page, &title, content_ref, on_delete)
}

fn setup_wiki_links(
    content_ref: NodeRef,
    navigator: Navigator,
    storage: StorageContext,
    title: &str,
) {
    let title = title.to_string();
    use_effect_with(title, move |_| {
        let Some(element) = content_ref.cast::<HtmlElement>() else {
            return;
        };
        let links = element.query_selector_all("a.wiki-link").unwrap();
        for i in 0..links.length() {
            let link: HtmlElement = links.get(i).unwrap().unchecked_into();
            apply_red_link(&link, &storage);
            attach_click_handler(&link, navigator.clone(), storage.clone());
        }
    });
}

fn apply_red_link(link: &HtmlElement, storage: &StorageContext) {
    if let Some(target) = link.get_attribute("data-wiki-link").as_deref()
        && !storage.has_page(target)
    {
        let _ = link.class_list().add_1("wiki-link-missing");
    }
}

fn attach_click_handler(link: &HtmlElement, nav: Navigator, stor: StorageContext) {
    let closure = gloo::events::EventListener::new(link, "click", move |e| {
        e.prevent_default();
        let target: HtmlElement = e.target().unwrap().unchecked_into();
        if let Some(wiki_target) = target.get_attribute("data-wiki-link").as_deref() {
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

fn build_delete_callback(
    title: String,
    storage: StorageContext,
    navigator: Navigator,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        storage.delete_page(&title);
        navigator.push(&Route::ViewPage {
            title: "MainPage".to_string(),
        });
    })
}

fn render_page(
    page: Option<wiki_common::model::WikiPage>,
    title: &str,
    content_ref: NodeRef,
    on_delete: Callback<MouseEvent>,
) -> Html {
    match page {
        Some(page) => {
            let rendered = render_wiki_content(&page.content);
            let is_main = page.title == "MainPage";
            let tier = aging::age_tier(page.updated_at, time::now());
            let age_class = format!("page-content {}", tier.css_class());
            html! {
                <div>
                    <h1 class="page-title">{&page.title}</h1>
                    <div ref={content_ref} class={age_class}>
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
                    <h1 class="page-title">{title}</h1>
                    <p>{"This page does not exist yet."}</p>
                    <Link<Route> to={Route::EditPage { title: title.to_string() }}>{"Create this page"}</Link<Route>>
                </div>
            }
        }
    }
}
