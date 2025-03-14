use wasm_bindgen::prelude::*;

///
/// # Refrences
///   <https://rustwasm.github.io/wasm-bindgen/examples/console-log.html>
///
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (crate::console::log(&format_args!($($t)*).to_string()))
}
pub(crate) use console_log;
