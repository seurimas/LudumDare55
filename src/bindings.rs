#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn get_clipboard_text_js() -> String;

    pub fn set_clipboard_text_js(text: &str);

    pub fn show_clipboard(top: &str, left: &str);

    pub fn hide_clipboard();

    pub fn save_game_js(name: String, json: String);

    pub fn show_load();

    pub fn hide_load();

    pub fn set_loader(f: &Closure<dyn FnMut(String)>);
}
