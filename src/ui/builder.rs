use std::sync::mpsc::Sender;

use crate::{
    input::structs::InputCommand,
    ui::structs::{KeyDef, Keyboard},
};

use gtk::{
    Application, ApplicationWindow, CssProvider, GestureClick, Grid, gdk, prelude::*,
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
    (base * SCALING_RATIO).floor() as i32
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

pub fn create_key(key: &KeyDef, tx: Sender<InputCommand>) -> gtk::Frame {
    let builder = gtk::Builder::from_file("resources/key.ui");

    let frame: gtk::Frame = builder.object("key_root").unwrap();

    let normal: gtk::Label = builder.object::<gtk::Label>("label_normal").unwrap();
    let shift: gtk::Label = builder.object::<gtk::Label>("label_shift").unwrap();

    normal.set_label(key.label(false));
    shift.set_label(key.label(true));

    let key_code = key.key_code();

    let gesture = GestureClick::new();

    {
        let tx = tx.clone();

        gesture.connect_pressed(move |_, _, _, _| {
            println!("Send Event DOWN:{:?}", key_code);
            let _ = tx.send(InputCommand::KeyDown(key_code));
        });
    }

    {
        let tx = tx.clone();

        gesture.connect_released(move |_, _, _, _| {
            println!("Send Event UP:{:?}", key_code);
            let _ = tx.send(InputCommand::KeyUp(key_code));
        });
    }

    frame.add_controller(gesture);

    frame
}

pub fn build_ui(app: &Application, keyboard: &Keyboard, tx: Sender<InputCommand>) {
    println!("WAYLAND_DISPLAY={:?}", std::env::var("WAYLAND_DISPLAY"));
    println!("XDG_SESSION_TYPE={:?}", std::env::var("XDG_SESSION_TYPE"));
    println!("LayerShell supported={}", gtk4_layer_shell::is_supported());

    load_css();

    let builder = gtk::Builder::from_file("resources/main.ui");

    let window: ApplicationWindow = builder.object::<ApplicationWindow>("main_window").unwrap();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    window.set_anchor(Edge::Bottom, true);

    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);

    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(0);

    window.connect_map(|window| {
        println!("window size: {}x{}", window.width(), window.height());
    });

    window.set_namespace(Some("osk"));

    let grid: Grid = builder.object::<Grid>("grid").unwrap();
    grid.set_row_spacing(ROW_SPACE);
    grid.set_column_spacing(COL_SPACE);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::End);

    window.set_application(Some(app));

    let mut c_num: i32;
    let mut r_num: i32 = 0;

    for line in keyboard.rows.iter() {
        c_num = 0;
        let mut rn_temp = i32::MAX;

        for key in line.keys.iter() {
            //dbg!(&key);
            //dbg!(&c_num);
            //dbg!(&r_num);

            let fixed_w = key_scaling(key.width());
            let fixed_h = key_scaling(key.height());
            let btn = create_key(key, tx.clone());

            btn.set_size_request(
                calc_length(CLTag::Width, key.width()),
                calc_length(CLTag::Height, key.height()),
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

        r_num += rn_temp;
    }

    window.present();
}
