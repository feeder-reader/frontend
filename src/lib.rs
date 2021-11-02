mod utils;
mod backend;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use feeder_types::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const NAME: &str = "Feeder";
type JsResult<T> = Result<T, JsValue>;

#[wasm_bindgen]
pub async fn start() -> JsResult<()> {
    utils::set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    class(&body, "bg-gray-200")?;

    let navbar = make_navbar(&document)?;
    body.append_child(&navbar)?;

    let content = make_content(&document, &window).await?;
    body.append_child(&content)?;

    Ok(())
}

async fn make_content(doc: &Document, window: &Window) -> JsResult<Element> {
    let container = doc.create_element("div")?;
    class(&container, "container mx-auto")?;

    let heading = doc.create_element("h1")?;
    heading.set_text_content(Some("All Feeds"));
    class(&heading, "text-4xl my-10 mx-5 font-bold")?;
    container.append_child(&heading)?;

    let entries = backend::update(&window).await?;
    let entries_list = make_entry_list(&doc, &entries)?;

    container.append_child(&entries_list)?;
    Ok(container)
}

fn make_entry_list(doc: &Document, entries: &[Entry]) -> JsResult<Element> {
    let entry_list = doc.create_element("ul")?;
    for entry in entries {
        let entry_elem = make_entry(&doc, &entry)?;
        entry_list.append_child(&entry_elem)?;
    }
    Ok(entry_list)
}

fn make_entry(doc: &Document, entry: &Entry) -> JsResult<Element> {
    let entry_container = doc.create_element("li")?;
    let entry_card = doc.create_element("div")?;
    let border = if entry.new { "border-2 border-red-300" } else { "" };
    let card_class = format!("{} {}", 
            "p-6 bg-white shadow-lg rounded-lg my-10",
            border);
    class(&entry_card, 
        &card_class)?;

    let title = doc.create_element("a")?;
    title.set_attribute("href", &entry.link)?;
    title.set_text_content(Some(&entry.title));
    class(&title, "text-gray-800 text-base font-semibold")?;
    entry_card.append_child(&title)?;

    let summary = doc.create_element("p")?;
    summary.set_text_content(Some(&entry.summary));
    class(&summary, "mt-2 text-gray-600 text-sm")?;
    entry_card.append_child(&summary)?;

    let details_class = "text-sm font-medium text-gray-400 px-2";

    let source_container = doc.create_element("div")?;
    class(&source_container,
        "flex justify-end mt-2")?;
    let source_text = doc.create_element("p")?;
    class(&source_text, details_class)?;
    source_text.set_text_content(Some(&entry.source));

    let date = doc.create_element("p")?;
    class(&date, details_class)?;
    date.set_text_content(Some("12.12.2021 14:55"));

    source_container.append_child(&source_text)?;
    source_container.append_child(&date)?;

    entry_card.append_child(&source_container)?;

    entry_container.append_child(&entry_card)?;
    Ok(entry_container)
}

fn make_navbar(doc: &Document) -> JsResult<Element> {
    let nav = doc.create_element("nav")?;
    class(&nav, "bg-black")?;
    
    let container = doc.create_element("div")?;
    class(&container, "max-w-7xl mx-auto px-2 sm:px-6 lg:px-8")?;

    let text = doc.create_element("h1")?;
    class(&text, "text-lg text-white p-2")?;
    text.set_text_content(Some(NAME));

    container.append_child(&text)?;
    nav.append_child(&container)?;
    Ok(nav)
}

fn class(elem: &Element, class: &str) -> JsResult<()> {
    elem.set_attribute("class", class)
}
