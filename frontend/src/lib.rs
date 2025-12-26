use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

mod app;
mod dashboard;
mod http;
mod settings;
mod state;
mod types;
mod websocket;

pub use http::HttpService;
pub use state::{app_state_reducer, AppAction, AppState};
pub use types::*;
pub use websocket::WebSocketService;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
    Ok(())
}
