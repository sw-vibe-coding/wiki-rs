use crate::storage::ExportFileStorage;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, HtmlInputElement, Url};
use wiki_common::model::WikiPage;
use wiki_ui::StorageContext;
use yew::prelude::*;

fn export_json(storage: &ExportFileStorage) -> String {
    let pages: Vec<WikiPage> = storage.pages.borrow().values().cloned().collect();
    serde_json::to_string_pretty(&pages).unwrap_or_default()
}

fn import_json(storage: &ExportFileStorage, json: &str) -> Result<usize, String> {
    let pages: Vec<WikiPage> =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {e}"))?;
    let count = pages.len();
    let mut store = storage.pages.borrow_mut();
    for page in pages {
        store.insert(page.title.clone(), page);
    }
    Ok(count)
}

fn downcast_storage(ctx: &StorageContext) -> &ExportFileStorage {
    let ptr = &*ctx.0 as *const dyn wiki_common::storage::WikiStorage;
    unsafe { &*(ptr as *const ExportFileStorage) }
}

fn trigger_download(json: &str) {
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(json));
    let opts = BlobPropertyBag::new();
    opts.set_type("application/json");
    if let Ok(blob) = Blob::new_with_str_sequence_and_options(&parts, &opts)
        && let Ok(url) = Url::create_object_url_with_blob(&blob)
    {
        let document = gloo::utils::document();
        if let Ok(elem) = document.create_element("a") {
            let anchor: HtmlAnchorElement = elem.unchecked_into();
            anchor.set_href(&url);
            anchor.set_download("wiki-pages.json");
            anchor.click();
            let _ = Url::revoke_object_url(&url);
        }
    }
}

#[function_component(ExportImportToolbar)]
pub fn export_import_toolbar() -> Html {
    let storage = use_context::<StorageContext>().unwrap();
    let file_input_ref = use_node_ref();
    let status = use_state(String::new);

    let on_export = {
        let storage = storage.clone();
        Callback::from(move |_| {
            let json = export_json(downcast_storage(&storage));
            trigger_download(&json);
        })
    };

    let on_import_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_file_selected = {
        let storage = storage.clone();
        let status = status.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            read_and_import(&input, &storage, &status);
            input.set_value("");
        })
    };

    html! {
        <div class="toolbar">
            <button onclick={on_export}>{"Export JSON"}</button>
            <button onclick={on_import_click}>{"Import JSON"}</button>
            <input
                ref={file_input_ref}
                type="file"
                accept=".json"
                style="display:none"
                onchange={on_file_selected}
            />
            if !status.is_empty() {
                <span class="toolbar-status">{&*status}</span>
            }
        </div>
    }
}

fn read_and_import(
    input: &HtmlInputElement,
    storage: &StorageContext,
    status: &UseStateHandle<String>,
) {
    if let Some(files) = input.files()
        && let Some(file) = files.get(0)
    {
        let reader = web_sys::FileReader::new().unwrap();
        let reader_clone = reader.clone();
        let storage = storage.clone();
        let status = status.clone();
        let onload = Closure::wrap(Box::new(move || {
            if let Ok(result) = reader_clone.result()
                && let Some(text) = result.as_string()
            {
                match import_json(downcast_storage(&storage), &text) {
                    Ok(count) => status.set(format!("Imported {count} pages")),
                    Err(err) => status.set(format!("Import error: {err}")),
                }
            }
        }) as Box<dyn Fn()>);
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        let _ = reader.read_as_text(&file);
    }
}
