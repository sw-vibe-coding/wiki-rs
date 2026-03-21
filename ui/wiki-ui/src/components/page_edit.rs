use crate::{Route, StorageContext};
use web_sys::HtmlTextAreaElement;
use wiki_common::model::WikiPage;
use wiki_common::time;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(PageEdit)]
pub fn page_edit(props: &Props) -> Html {
    let storage = use_context::<StorageContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let title = props.title.clone();

    let existing = storage.get_page(&title);
    let created_at = existing.as_ref().map(|p| p.created_at).unwrap_or(0);
    let initial_content = existing.map(|p| p.content).unwrap_or_default();
    let content = use_state(|| initial_content.clone());

    let on_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(textarea.value());
        })
    };

    let on_save = build_save_callback(
        content.clone(),
        title.clone(),
        created_at,
        storage,
        navigator.clone(),
    );
    let on_cancel = build_cancel_callback(title.clone(), navigator);
    let is_new = initial_content.is_empty();
    let heading = if is_new {
        format!("Creating: {title}")
    } else {
        format!("Editing: {title}")
    };

    html! {
        <div>
            <h1 class="page-title">{heading}</h1>
            <p class="edit-hint">{"Use [[PageName]] to link to other wiki pages."}</p>
            <textarea value={(*content).clone()} oninput={on_input}/>
            <div>
                <button onclick={on_save}>{"Save"}</button>
                <button onclick={on_cancel}>{"Cancel"}</button>
            </div>
        </div>
    }
}

fn build_save_callback(
    content: UseStateHandle<String>,
    title: String,
    created_at: u64,
    storage: StorageContext,
    navigator: Navigator,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let now = time::now();
        let ca = if created_at == 0 { now } else { created_at };
        storage.save_page(WikiPage {
            title: title.clone(),
            content: (*content).clone(),
            created_at: ca,
            updated_at: now,
        });
        navigator.push(&Route::ViewPage {
            title: title.clone(),
        });
    })
}

fn build_cancel_callback(title: String, navigator: Navigator) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        navigator.push(&Route::ViewPage {
            title: title.clone(),
        });
    })
}
