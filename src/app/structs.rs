use crate::{
    config::{
        loader::load_config,
        structs::{Config, UiPlace},
    },
    event::log::Logger,
    input::structs::InputCommand,
    ui::{
        builder::build_ui,
        load::load_keyboard,
        structs::{KeyComponentsTable, Keyboard},
    },
};
use evdevil::event::Key;
use gtk::{Application, ApplicationWindow, prelude::*};
use gtk4_layer_shell::{Edge, LayerShell};
use std::{
    path::PathBuf,
    sync::{Arc, mpsc::Sender},
};

pub struct UiState {
    window: ApplicationWindow,
    kct: KeyComponentsTable,
    current_ui_place: UiPlace,
    logger: Arc<Logger>,
}
impl UiState {
    pub fn new(
        app: &Application,
        input_state: &InputState,
        tx_ic: &Sender<InputCommand>,
        logger: Arc<Logger>,
    ) -> Self {
        let keyboard = &input_state.keyboard;
        let config = &input_state.config;

        let mut kct = KeyComponentsTable::new();

        let window = build_ui(
            app,
            keyboard,
            &mut kct,
            tx_ic.clone(),
            &config.default_monitor,
            &config.default_ui_view,
            &config.default_ui_place,
            Arc::clone(&logger),
        );

        let current_ui_place = config.default_ui_place.clone();

        Self {
            window,
            kct,
            current_ui_place,
            logger,
        }
    }

    pub fn window_set_visible(&self, new_visible: bool) {
        self.logger
            .trace(format!("Visible changing to: {new_visible}"));
        self.window.set_visible(new_visible);
    }

    pub fn window_get_visible(&self) -> bool {
        self.window.is_visible()
    }

    pub fn window_set_anchor(&self, new_anchor: Edge) {
        self.window.set_anchor(Edge::Bottom, false);
        self.window.set_anchor(Edge::Top, false);

        self.logger
            .trace(format!("Anchor changing to: {:?}", new_anchor));

        self.window.set_anchor(new_anchor, true);
    }

    pub fn window_close(&self) {
        self.logger.trace("Window close");
        self.window.close();
    }

    pub fn set_ui_place(&mut self, new_place: UiPlace) {
        self.logger
            .trace(format!("UI place changing to: {:?}", new_place));
        self.current_ui_place = new_place;
    }

    pub fn get_ui_place(&self) -> UiPlace {
        self.current_ui_place.clone()
    }

    pub fn kct_set_text(&mut self, addr: (usize, usize), texts: (&str, &str, &str)) {
        self.logger
            .trace(format!("Key({:?}) texts changing to: {:?}", addr, texts));
        self.kct.set_text(addr, texts);
    }
    pub fn kct_add_css_class(&mut self, addr: (usize, usize), class_name: &str) {
        self.logger
            .trace(format!("Key({:?}) added CSS class: {:?}", addr, class_name));
        self.kct.add_css_class(addr, class_name);
    }
    pub fn kct_rmv_css_class(&mut self, addr: (usize, usize), class_name: &str) {
        self.logger.trace(format!(
            "Key({:?}) removed CSS class: {:?}",
            addr, class_name
        ));
        self.kct.rmv_css_class(addr, class_name);
    }
}

pub struct InputState {
    keyboard: Keyboard,
    config: Config,
    logger: Arc<Logger>,
}
impl InputState {
    pub fn new(custom_path: &Option<PathBuf>, logger: Arc<Logger>) -> Self {
        let config = load_config(custom_path, Arc::clone(&logger));
        let keyboard = load_keyboard(&config.layout);

        Self {
            keyboard,
            config,
            logger,
        }
    }

    pub fn kb_supperted_keys(&self) -> Vec<Key> {
        self.keyboard.supperted_keys()
    }

    pub fn get_kb_ref(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn get_conf_ref(&self) -> &Config {
        &self.config
    }

    pub fn set_monitor_name(&mut self, new_name: &str) {
        self.logger.trace(format!(
            "Monitor connection name changing to: {:?}",
            new_name
        ));
        self.config.set_monitor_name(new_name);
    }

    pub fn get_monitor_name(&self) -> String {
        self.config.get_monitor_name()
    }
}
