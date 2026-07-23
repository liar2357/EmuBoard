use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub layout: String,
    pub is_hold_modify: bool,
    pub default_monitor: String,
    pub default_ui_view: bool,
    pub default_ui_place: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: "JIS-QWERTY".to_string(),
            is_hold_modify: false,
            default_monitor: "auto".to_string(),
            default_ui_view: true,
            default_ui_place: "lower".to_string(),
        }
    }
}
