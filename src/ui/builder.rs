use std::sync::{Arc, mpsc::Sender};

use crate::{
    config::structs::UiPlace,
    input::structs::InputCommand,
    ui::{
        monitor::setup_monitor,
        structs::{KeyComponentsTable, KeyDef, Keyboard},
    },
};

use gtk::{
    Application, ApplicationWindow, CssProvider, GestureClick, Grid, gdk, prelude::*,
    style_context_add_provider_for_display,
};

use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const SCALING_UNIT_COL: i32 = 5;
const SCALING_UNIT_ROW: i32 = 3;
const COL_SPACE: i32 = 3;
const ROW_SPACE: i32 = 3;

enum CLTag {
    Width,
    Height,
}

fn calc_key_base_scale(kb: Arc<Keyboard>, global_width: i32) -> i32 {
    let all_unit_in_line = kb.calc_key_unit_in_line();
    let col_space_sum = COL_SPACE * (all_unit_in_line - 1);

    dbg!(((dbg!(global_width) - col_space_sum) / dbg!(all_unit_in_line) / SCALING_UNIT_COL) as i32)
}

fn key_scaling(base: i32, scale: i32) -> i32 {
    base * scale
}

fn calc_length(mode: CLTag, base: i32, scale: i32) -> i32 {
    let key_length = key_scaling(base, scale)
        * if let CLTag::Height = mode {
            SCALING_UNIT_ROW
        } else {
            SCALING_UNIT_COL
        };

    let space_length = (base - 1)
        * if let CLTag::Height = mode {
            COL_SPACE
        } else {
            ROW_SPACE
        };

    key_length + space_length
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_resource("/io/github/liar2357/emu-board/css/style.css");

    style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn create_key(
    key: &KeyDef,
    key_addr: (usize, usize),
    tx: Sender<InputCommand>,
) -> (gtk::Frame, gtk::Label, gtk::Label, gtk::Label) {
    let builder = gtk::Builder::from_resource("/io/github/liar2357/emu-board/ui/key.ui");

    let frame: gtk::Frame = builder.object("key_root").unwrap();

    let normal: gtk::Label = builder.object::<gtk::Label>("label_normal").unwrap();
    let shift: gtk::Label = builder.object::<gtk::Label>("label_shift").unwrap();
    let func: gtk::Label = builder.object::<gtk::Label>("label_func").unwrap();

    normal.set_label(key.label(false, false));
    shift.set_label(key.label(true, false));
    func.set_label(key.label(false, true));

    let gesture = GestureClick::new();

    {
        let tx = tx.clone();

        gesture.connect_pressed(move |_, _, _, _| {
            let _ = tx.send(InputCommand::KeyDown(key_addr));
        });
    }

    {
        let tx = tx.clone();

        gesture.connect_released(move |_, _, _, _| {
            let _ = tx.send(InputCommand::KeyUp(key_addr));
        });
    }

    frame.add_controller(gesture);

    (frame, normal, shift, func)
}

pub fn build_ui(
    app: &Application,
    keyboard: Arc<Keyboard>,
    kct: &mut KeyComponentsTable,
    tx: Sender<InputCommand>,
    default_monitor: &str,
    default_ui_view: &bool,
    default_ui_place: &UiPlace,
) -> ApplicationWindow {
    eprintln!("WAYLAND_DISPLAY={:?}", std::env::var("WAYLAND_DISPLAY"));
    eprintln!("XDG_SESSION_TYPE={:?}", std::env::var("XDG_SESSION_TYPE"));
    eprintln!("LayerShell supported={}", gtk4_layer_shell::is_supported());

    load_css();

    let builder = gtk::Builder::from_resource("/io/github/liar2357/emu-board/ui/main.ui");

    let window: ApplicationWindow = builder.object::<ApplicationWindow>("main_window").unwrap();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    window.set_anchor(
        match default_ui_place {
            UiPlace::Upper => Edge::Top,
            UiPlace::Lower => Edge::Bottom,
        },
        true,
    );

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(0);

    window.set_namespace(Some(env!("CARGO_PKG_NAME")));

    let global_width = setup_monitor(&window, default_monitor).unwrap_or(1200);
    let key_base_scale = calc_key_base_scale(keyboard.clone(), global_width);

    let grid: Grid = builder.object::<Grid>("grid").unwrap();
    grid.set_row_spacing(ROW_SPACE as u32);
    grid.set_column_spacing(COL_SPACE as u32);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::End);

    window.set_application(Some(app));

    let mut c_num: i32;
    let mut r_num: i32 = 0;

    for (r, line) in keyboard.rows.iter().enumerate() {
        c_num = 0;
        let mut rn_temp = i32::MAX;

        for (c, key) in line.keys.iter().enumerate() {
            let fixed_w = key_scaling(key.width(), key_base_scale);
            let fixed_h = key_scaling(key.height(), key_base_scale);
            let (btn, l1, l2, l3) = create_key(key, (r, c), tx.clone());

            btn.set_size_request(
                calc_length(CLTag::Width, key.width(), key_base_scale),
                calc_length(CLTag::Height, key.height(), key_base_scale),
            );

            btn.set_hexpand(false);
            btn.set_vexpand(false);

            btn.set_halign(gtk::Align::Center);
            btn.set_valign(gtk::Align::Center);

            grid.attach(&btn, c_num, r_num, fixed_w, fixed_h);

            kct.append((r, c), (btn, l1, l2, l3));

            c_num += fixed_w;
            if rn_temp > fixed_h {
                rn_temp = fixed_h;
            }
        }

        r_num += rn_temp;
    }

    window.present();
    window.set_visible(*default_ui_view);

    window
}
