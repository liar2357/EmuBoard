use crate::ui::structs::Keyboard;

use gtk::gio::{ResourceLookupFlags, resources_lookup_data};

pub fn load_keyboard(layout: &str) -> Keyboard {
    let resource = format!("/io/github/liar2357/emu-board/layout/{layout}.toml");

    let bytes = resources_lookup_data(&resource, ResourceLookupFlags::NONE)
        .expect("Failed to load resource");

    let text = std::str::from_utf8(bytes.as_ref()).expect("Invalid UTF-8");

    toml::from_str(text).expect("Failed to parse TOML")
}
