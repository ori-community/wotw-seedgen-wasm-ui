use wasm_bindgen::prelude::*;
use js_sys::Function;

use wotw_seedgen::files::FileAccess;

/// Interface to serve files as needed
#[wasm_bindgen]
pub struct JsFileAccess {
    game_preset_callback: Function,
    world_preset_callback: Function,
    header_callback: Function,
}
impl FileAccess for JsFileAccess {
    fn read_game_preset(&self, identifier: &str) -> Result<String, String> {
        js_call(&self.game_preset_callback, identifier)
    }
    fn read_world_preset(&self, identifier: &str) -> Result<String, String> {
        js_call(&self.world_preset_callback, identifier)
    }
    fn read_header(&self, identifier: &str) -> Result<String, String> {
        js_call(&self.header_callback, identifier)
    }
}
fn js_call(function: &Function, identifier: &str) -> Result<String, String> {
    function.call1(&JsValue::null(), &JsValue::from_str(identifier))
        .map_err(|err| format!("callback threw: {}", err.as_string().unwrap_or_else(|| format!("{err:?}"))))
        .and_then(|ok| ok.as_string().ok_or_else(|| format!("callback did not return a string: {ok:?}")))
}

#[wasm_bindgen]
impl JsFileAccess {
    /// Creates a new `JsFileAccess` using the given callbacks
    /// 
    /// Callbacks should follow the signature `(identifier: string) => string` (`identifier` would e.g. be "gorlek" when requesting the world preset) and may throw
    /// 
    /// This type will have to be passed when working with presets or headers since they may include further files
    #[wasm_bindgen(constructor)]
    pub fn new(
        game_preset_callback: Function,
        world_preset_callback: Function,
        header_callback: Function,
    ) -> Self {
        Self { game_preset_callback, world_preset_callback, header_callback }
    }
}
