use wasm_bindgen::prelude::*;
pub mod app;
mod dummy;
pub mod pages;
pub mod support;
mod switch;
use dummy::Dummy;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Dummy>();
    Ok(())
}
