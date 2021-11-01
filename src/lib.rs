mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use feeder_types::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const NAME: &str = "Feeder";


#[wasm_bindgen]
pub async fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    class(&body, "bg-gray-800")?;

    let navbar = make_navbar(&document)?;
    body.append_child(&navbar)?;

    let content = make_content(&document)?;
    body.append_child(&content)?;

    Ok(())
}

fn make_content(doc: &Document) -> Result<Element, JsValue> {
    let container = doc.create_element("div")?;
    class(&container, "container mx-auto")?;

    let val = doc.create_element("p")?;
    val.set_text_content(Some("This is a test LOL"));
    val.set_attribute("class", "text-gray-600")?;
    container.append_child(&val)?;
    Ok(container)
}

fn make_navbar(doc: &Document) -> Result<Element, JsValue> {
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

fn class(elem: &Element, class: &str) -> Result<(), JsValue> {
    elem.set_attribute("class", class)
}
