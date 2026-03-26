use crate::ui::structs::Keyboard;
use std::fs;

pub fn load_keyboard(layout: &str) -> Keyboard {
    let text = fs::read_to_string(format!("resources/{layout}.toml")).expect("Failed to read file");

    toml::from_str(&text).expect("Failed to parse TOML")
}
