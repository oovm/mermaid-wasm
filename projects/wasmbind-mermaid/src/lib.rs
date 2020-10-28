use wasm_bindgen::prelude::*;
use web_sys::Element;
mod options;
pub use options::{FlowChartConfig, MermaidOptions, SequenceDiagramConfig};

#[wasm_bindgen(module = "/src/mermaid.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = draw_mermaid)]
    pub fn draw_mermaid(e: &Element, input: &str, cfg: &JsValue) -> String;
}
