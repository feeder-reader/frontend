use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use crate::JsResult;
use feeder_types::*;

pub async fn update(window: &Window) -> JsResult<Vec<Entry>> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    //opts.mode(RequestMode::Cors);

    let url = "http://localhost:3344/updates";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    let entries: Vec<Entry> = json.into_serde().unwrap();
    Ok(entries)
}
