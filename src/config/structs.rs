use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum HoldMode {
    None,
    Hold,
    Toggle,
    HoldAndToggle,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UiPlace {
    Upper,
    Lower,
}

#[derive(Debug, Clone)]
pub enum UiScale {
    Pixel(i32),
    Percent(i32),
}
impl Serialize for UiScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            UiScale::Pixel(value) => format!("{value}px"),
            UiScale::Percent(value) => format!("{value}%"),
        };

        serializer.serialize_str(&value)
    }
}
impl<'de> Deserialize<'de> for UiScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Some(value) = value.strip_suffix('%') {
            let value = value.parse::<i32>().map_err(serde::de::Error::custom)?;

            if !(1..=200).contains(&value) {
                return Err(serde::de::Error::custom("ui_scale avalavle 1% ~ 200%"));
            }

            return Ok(UiScale::Percent(value));
        }

        if let Some(value) = value.strip_suffix("px") {
            let value = value.parse::<i32>().map_err(serde::de::Error::custom)?;

            if value <= 0 {
                return Err(serde::de::Error::custom("ui_scale must be grater then 0px"));
            }

            return Ok(UiScale::Pixel(value));
        }

        Err(serde::de::Error::custom(
            "ui_scale must end with '%' or 'px'",
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    #[serde(default = "default_ui_height")]
    pub ui_height: UiScale,

    #[serde(default = "default_ui_width")]
    pub ui_width: UiScale,
}
impl Config {
    pub fn get_ref(&self) -> &Self {
        self
    }

    pub fn get_mut_ref(&mut self) -> &mut Self {
        self
    }

    pub fn set_monitor_name(&mut self, new_name: &str) {
        self.default_monitor = new_name.to_string();
    }

    pub fn get_monitor_name(&self) -> String {
        self.default_monitor.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: default_layout(),
            hold_mode: default_hold_mode(),
            default_monitor: default_default_monitor(),
            default_ui_view: default_default_ui_view(),
            default_ui_place: default_default_ui_place(),
            ui_height: default_ui_height(),
            ui_width: default_ui_width(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub configs: Vec<Config>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            configs: vec![Config::default()],
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
fn default_ui_height() -> UiScale {
    UiScale::Percent(30)
}
fn default_ui_width() -> UiScale {
    UiScale::Percent(100)
}
