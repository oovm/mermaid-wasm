use crate::draw_mermaid;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::Element;
mod flow_chart;
mod gantt;
mod sequence;

pub use flow_chart::FlowChartConfig;
pub use gantt::GanttConfig;
pub use sequence::SequenceDiagramConfig;

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize)]
/// Read <https://katex.org/docs/options.html> for more information.
pub struct MermaidOptions {
    /// Whether to render the math in the display mode.

    /// securityLevel: disallow/allow potentially dangerous cross-site scripting behavior
    ///   the two documented values are "strict" and "loose", i.e. disallow and allow
    ///   default: "strict"
    ///   If the value is not present, the default behavior is "strict"
    ///   Up through version mermaid@8.2.3, if any text value is present in a config but is not "strict", the behavior is "loose".
    ///   This should be fixed after that version, i.e. any value other "loose" should be treated as "strict".
    #[serde(rename = "securityLevel")]
    security_level: String,

    theme: String,

    /// logLevel , decides the amount of logging to be used.
    /// default: LogLevel.Fatal
    #[serde(rename = "logLevel")]
    log_level: u32,

    /// **startOnLoad** - This options controls whether or mermaid starts when the page loads
    /// default: true
    #[serde(rename = "startOnLoad")]
    pub start_on_load: bool,

    /// **arrowMarkerAbsolute** - This options controls whether or arrow markers in html code will be absolute paths or
    /// an anchor, #. This matters if you are using base tag settings.
    /// default: false
    #[serde(rename = "arrowMarkerAbsolute")]
    pub arrow_marker_absolute: bool,

    /// ### flowchart
    /// *The object containing configurations specific for flowcharts*
    #[serde(skip_serializing_if = "Option::is_none")]
    flowchart: Option<FlowChartConfig>,

    /// ###  sequenceDiagram
    /// The object containing configurations specific for sequence diagrams
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence: Option<SequenceDiagramConfig>,

    /// ### gantt
    /// The object containing configurations specific for gantt diagrams*
    #[serde(skip_serializing_if = "Option::is_none")]
    gantt: Option<GanttConfig>, /* class?: any;
                                 * git?: any; */
}

impl Default for MermaidOptions {
    fn default() -> Self {
        Self {
            security_level: "antiscript".to_string(),
            theme: "default".to_string(),
            log_level: 1,
            start_on_load: false,
            arrow_marker_absolute: false,
            flowchart: None,
            sequence: None,
            gantt: None,
        }
    }
}

impl MermaidOptions {
    pub fn set_theme(&mut self, new: &str) -> bool {
        match new {
            "default" | "forest" | "dark" | "neutral" => {
                self.theme = String::from(new);
                true
            }
            _ => false,
        }
    }
    pub fn set_security_level(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_log_level(&mut self) -> bool {
        unimplemented!()
    }
    pub fn render(&self, input: &Element, s: &str) -> String {
        draw_mermaid(input, s, &JsValue::from_serde(self).unwrap())
    }
}
