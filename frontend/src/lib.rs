use wasm_bindgen::prelude::*;
mod app;
mod pages;
mod switch;
use app::Index;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Index>();
    Ok(())
}




