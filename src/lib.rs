mod utils;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use gloo::{
    events::EventListener,
    render::{request_animation_frame, AnimationFrame},
};
use web_sys::{
    AudioBuffer, AudioContext, Event, HtmlCanvasElement, KeyboardEvent, WebGlBuffer, WebGlProgram,
    WebGlRenderingContext, WebGlShader, WebGlTexture, WebGlUniformLocation,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;
    let ctx_options = js_sys::Object::new();
    js_sys::Reflect::set(&ctx_options, &"alpha".into(), &false.into()).unwrap();
    let gl: WebGlRenderingContext = canvas
        .get_context_with_context_options("webgl", &ctx_options)?
        .unwrap()
        .dyn_into()?;

    Ok(())
}
