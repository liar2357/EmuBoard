use crate::ui::load::load_keyboard;
use crate::ui::structs::{KeyDef, Keyboard};
use gtk::CssProvider;
use gtk::gdk;
use gtk::prelude::*;
use gtk::style_context_add_provider_for_display;
use gtk::{Application, ApplicationWindow, Grid};

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_path("resources/style.css");

    style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn create_key(key: &KeyDef) -> gtk::Button {
    let builder = gtk::Builder::from_file("resources/key.ui");

    let button: gtk::Button = builder.object::<gtk::Button>("key_button").unwrap();

    let normal: gtk::Label = builder.object::<gtk::Label>("label_normal").unwrap();
    let shift: gtk::Label = builder.object::<gtk::Label>("label_shift").unwrap();

    normal.set_label(key.normal.as_str());
    shift.set_label(key.shift.as_str());

    let normal_val = key.normal.to_string();
    let shift_val = key.shift.to_string();

    button.connect_clicked(move |_| {
        println!("Pressed: {} / {}", normal_val, shift_val);
    });

    button
}

pub fn build_ui(app: &Application) {
    load_css();

    let builder = gtk::Builder::from_file("resources/main.ui");

    let window: ApplicationWindow = builder.object::<ApplicationWindow>("main_window").unwrap();

    let grid: Grid = builder.object::<Grid>("grid").unwrap();

    window.set_application(Some(app));

    let keyboard: Keyboard = load_keyboard("test");

    for (r_num, line) in keyboard.rows.iter().enumerate() {
        for (c_num, key) in line.keys.iter().enumerate() {
            let btn = create_key(key);

            grid.attach(&btn, c_num as i32, r_num as i32, 1, 1);
        }
    }

    window.present();
}
