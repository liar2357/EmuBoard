use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum HoldMode {
    None,
    Hold,
    Toggle,
}

#[derive(Debug, Clone, Deserialize)]
pub enum UiPlace {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_layout")]
    pub layout: String,

    #[serde(default = "default_hold_mode")]
    pub hold_mode: HoldMode,

    #[serde(default = "default_default_monitor")]
    pub default_monitor: String,

    #[serde(default = "default_default_ui_view")]
    pub default_ui_view: bool,

    #[serde(default = "default_default_ui_place")]
    pub default_ui_place: UiPlace,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: "JIS-QWERTY".to_string(),
            hold_mode: HoldMode::None,
            default_monitor: "auto".to_string(),
            default_ui_view: true,
            default_ui_place: UiPlace::Lower,
        }
    }
}

fn default_layout() -> String {
    "JIS-QWERTY".to_string()
}
fn default_hold_mode() -> HoldMode {
    HoldMode::None
}
fn default_default_monitor() -> String {
    "auto".to_string()
}
fn default_default_ui_view() -> bool {
    true
}
fn default_default_ui_place() -> UiPlace {
    UiPlace::Lower
}
