use crate::ui::load::load_keyboard;
use crate::ui::structs::{KeyDef, Keyboard};
use gtk::{
    Application, ApplicationWindow, CssProvider, Grid, gdk, prelude::*,
    style_context_add_provider_for_display,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const SCALING_RATIO: f32 = 8.0;
const SCALING_UNIT: i32 = 5;
const ROW_SPACE: u32 = 5;
const COL_SPACE: u32 = 5;

enum CLTag {
    Width,
    Height,
}

fn key_scaling(base: f32) -> i32 {
    dbg!((base * SCALING_RATIO).floor() as i32)
}

fn calc_length(mode: CLTag, base: f32) -> i32 {
    key_scaling(base) * SCALING_UNIT
        + (base - 1.0).floor() as i32
            * if let CLTag::Height = mode {
                ROW_SPACE
            } else {
                COL_SPACE
            } as i32
}

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

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(0);
    window.set_namespace(Some("osk"));

    let grid: Grid = builder.object::<Grid>("grid").unwrap();
    grid.set_row_spacing(ROW_SPACE);
    grid.set_column_spacing(COL_SPACE);

    window.set_application(Some(app));

    let keyboard: Keyboard = load_keyboard("JIS-QWERTY");

    let mut c_num: i32;
    let mut r_num: i32 = 0;

    for line in keyboard.rows.iter() {
        c_num = 0;
        let mut rn_temp = i32::MAX;

        for key in line.keys.iter() {
            dbg!(&key);
            dbg!(&c_num);
            dbg!(&r_num);

            let fixed_w = key_scaling(key.width);
            let fixed_h = key_scaling(key.height);
            let btn = create_key(key);

            btn.set_size_request(
                dbg!(calc_length(CLTag::Width, key.width)),
                dbg!(calc_length(CLTag::Height, key.height)),
            );

            btn.set_hexpand(false);
            btn.set_vexpand(false);

            btn.set_halign(gtk::Align::Center);
            btn.set_valign(gtk::Align::Center);

            grid.attach(&btn, c_num, r_num, fixed_w, fixed_h);

            c_num += fixed_w;
            if rn_temp > fixed_h {
                rn_temp = fixed_h;
            }
        }
        dbg!("---");

        r_num += rn_temp;
    }

    window.present();
}
