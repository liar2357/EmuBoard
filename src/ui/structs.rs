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
    pub shift: String,
}
