use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(input: &str) -> Result<String, JsError> {
    let opts = crate::ProcessOptions::default();
    crate::process_with_options(input, &opts)
        .map_err(|e| JsError::new(&format!("{:#}", e)))
}
