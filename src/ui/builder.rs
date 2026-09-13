#![allow(clippy::too_many_arguments)]

use crate::{
    app::structs::InputState,
    config::structs::{UiPlace, UiScale},
    event::log::Logger,
    input::structs::InputCommand,
    ui::{
        monitor::setup_monitor,
        structs::{KeyComponentsTable, KeyDef},
    },
};
use gtk::{
    Application, ApplicationWindow, CssProvider, GestureClick, Grid, gdk, prelude::*,
    style_context_add_provider_for_display,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::{
    collections::HashMap,
    sync::{Arc, mpsc::Sender},
};

const COL_SPACE: i32 = 3;
const ROW_SPACE: i32 = 3;

fn calc_key_base_scale(
    input_state: &InputState,
    (global_width, global_height): (i32, i32),
    logger: Arc<Logger>,
) -> (i32, i32) {
    let (kb_width, kb_height) = {
        (
            match input_state.get_conf_ref().ui_width {
                UiScale::Pixel(v) => v,
                UiScale::Percent(v) => (global_width as f64 * (v as f64 / 100.0)) as i32,
            },
            match input_state.get_conf_ref().ui_height {
                UiScale::Pixel(v) => v,
                UiScale::Percent(v) => (global_height as f64 * (v as f64 / 100.0)) as i32,
            },
        )
    };

    let all_unit_in_line = input_state.get_kb_ref().calc_key_unit_in_line();
    let row_num_in_kb = input_state.get_kb_ref().get_rows_num();

    let col_space_sum = COL_SPACE * (all_unit_in_line - 1);
    let row_space_sum = ROW_SPACE * (row_num_in_kb - 1);

    let key_base_scale_w = (kb_width - col_space_sum) / all_unit_in_line;
    let key_base_scale_h = (kb_height - row_space_sum) / row_num_in_kb;

    logger.info(format!("monitor_scale = {global_width}x{global_height}"));
    logger.info(format!(
        "key_base_scale = ({key_base_scale_w},{key_base_scale_h})"
    ));

    (key_base_scale_w, key_base_scale_h)
}

fn key_scaling(base: i32, scale: i32) -> i32 {
    base * scale
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
    input_state: &InputState,
    kct: &mut KeyComponentsTable,
    tx: Sender<InputCommand>,
    logger: Arc<Logger>,
) -> ApplicationWindow {
    logger.info(format!(
        "WAYLAND_DISPLAY={:?}",
        std::env::var("WAYLAND_DISPLAY")
    ));
    logger.info(format!(
        "XDG_SESSION_TYPE={:?}",
        std::env::var("XDG_SESSION_TYPE")
    ));
    if gtk4_layer_shell::is_supported() {
        logger.info("LayerShell supported=true");
    } else {
        logger.warn("LayerShell supported=false");
    }

    load_css();

    let builder = gtk::Builder::from_resource("/io/github/liar2357/emu-board/ui/main.ui");

    let window: ApplicationWindow = builder.object::<ApplicationWindow>("main_window").unwrap();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    window.set_anchor(
        match input_state.get_conf_ref().default_ui_place {
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

    let (global_width, global_height) = match setup_monitor(
        &window,
        &input_state.get_conf_ref().default_monitor,
        Arc::clone(&logger),
    ) {
        Some(v) => v,
        None => {
            logger.warn("Failed to get monitor width and height");
            (1200, 720)
        }
    };

    let (key_base_scale_w, key_base_scale_h) = calc_key_base_scale(
        input_state,
        (global_width, global_height),
        Arc::clone(&logger),
    );

    let grid: Grid = builder.object::<Grid>("grid").unwrap();
    grid.set_row_spacing(ROW_SPACE as u32);
    grid.set_column_spacing(COL_SPACE as u32);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::End);

    window.set_application(Some(app));

    let mut tall_buc: HashMap<i32, Vec<i32>> = HashMap::new();

    for (r, line) in input_state.get_kb_ref().rows.iter().enumerate() {
        let mut c_num = 0;

        for (c, key) in line.keys.iter().enumerate() {
            let fixed_w = key_scaling(key.width(), key_base_scale_w);
            let fixed_h = key_scaling(key.height(), key_base_scale_h);
            let (btn, l1, l2, l3) = create_key(key, (r, c), tx.clone());

            btn.set_size_request(fixed_w, fixed_h);

            btn.set_hexpand(false);
            btn.set_vexpand(false);

            btn.set_halign(gtk::Align::Fill);
            btn.set_valign(gtk::Align::Fill);

            btn.set_margin_start(0);
            btn.set_margin_end(0);
            btn.set_margin_top(0);
            btn.set_margin_bottom(0);

            let appends: Vec<i32> = (c_num..c_num + key.width()).collect();
            if key.height() > 1 {
                for i in r as i32 + 1..r as i32 + key.height() {
                    if let std::collections::hash_map::Entry::Vacant(e) = tall_buc.entry(i) {
                        e.insert(appends.clone());
                    } else {
                        tall_buc.get_mut(&i).unwrap().extend(appends.clone());
                    }
                }
            }

            while let Some(v) = tall_buc.get(&(r as i32)) {
                if v.contains(&c_num) {
                    c_num += 1;
                } else {
                    break;
                }
            }

            grid.attach(&btn, c_num, r as i32, key.width(), key.height());
            logger.trace(format!(
                "key({}) created and setting at ({r},{c_num})",
                key.label(false, false)
            ));
            c_num += key.width();

            kct.append((r, c), (btn, l1, l2, l3));
        }
    }

    window.present();

    window.set_visible(input_state.get_conf_ref().default_ui_view);
    window
}
