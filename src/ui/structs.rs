use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub rows: Vec<Line>,
}

#[derive(Debug, Deserialize)]
pub struct Line {
    pub keys: Vec<KeyDef>,
}

#[derive(Debug, Deserialize)]
pub struct KeyDef {
    pub normal: String,

    #[serde(default = "default_shift")]
    pub shift: String,

    #[serde(default = "default_width")]
    pub width: f32,

    #[serde(default = "default_height")]
    pub height: f32,
}

fn default_shift() -> String {
    "".to_string()
}

fn default_width() -> f32 {
    1.0
}

fn default_height() -> f32 {
    1.0
}
