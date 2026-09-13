use crate::{
    app::structs::InputState,
    config::structs::HoldMode,
    event::{log::Logger, structs::UiEvent},
    ui::structs::{CustomKey, KeyDef, KeyWrap, StyleCtl},
};
use evdevil::{
    event::{KeyEvent, KeyState},
    uinput::UinputDevice,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, mpsc::Sender},
};

#[derive(Debug, Clone, Copy)]
pub enum InputCommand {
    KeyDown((usize, usize)),
    KeyUp((usize, usize)),
    Shutdown,
}

pub struct InputSender {
    device: UinputDevice,
    input_state: Arc<RwLock<InputState>>,
    ui_eve_sender: Sender<UiEvent>,

    is_fn: bool,

    modifier_map: HashMap<String, u8>,

    logger: Arc<Logger>,
}

impl InputSender {
    pub fn new(
        input_state: Arc<RwLock<InputState>>,
        ui_eve_sender: Sender<UiEvent>,
        logger: Arc<Logger>,
    ) -> anyhow::Result<Self> {
        let device = UinputDevice::builder()?
            .with_keys(input_state.read().unwrap().kb_supperted_keys())?
            .build(env!("CARGO_PKG_NAME"))?;

        Ok(Self {
            device,
            input_state,
            ui_eve_sender,
            is_fn: false,
            modifier_map: {
                let mut map = HashMap::new();
                map.insert("LAlt".to_string(), 0);
                map.insert("RAlt".to_string(), 0);
                map.insert("LCtrl".to_string(), 0);
                map.insert("RCtrl".to_string(), 0);
                map.insert("LShift".to_string(), 0);
                map.insert("RShift".to_string(), 0);
                map.insert("LSuper".to_string(), 0);
                map.insert("RSuper".to_string(), 0);
                map.insert("Fn".to_string(), 0);
                map
            },
            logger,
        })
    }

    pub fn key_down(&mut self, key_addr: (usize, usize)) -> std::io::Result<()> {
        let is_g = self.input_state.read().unwrap();
        let kb_g = is_g.get_kb_ref();
        let conf_g = is_g.get_conf_ref();

        let key_ref = kb_g.get_keydef_by_addr(key_addr);

        match key_ref.key_code(self.is_fn) {
            KeyWrap::Default(key) => {
                self.logger.trace(format!("PRESSED:{:?}", key));

                let command = match conf_g.hold_mode {
                    HoldMode::None => Some(KeyState::PRESSED),
                    HoldMode::Hold | HoldMode::Toggle => {
                        if key_ref.is_modifier() && self.modifier_map[&key_ref.get_key_name()] == 1
                        {
                            self.modifier_map.insert(key_ref.get_key_name(), 0);

                            Some(KeyState::RELEASED)
                        } else {
                            if key_ref.is_modifier() {
                                self.modifier_map.insert(key_ref.get_key_name(), 1);
                            }
                            Some(KeyState::PRESSED)
                        }
                    }
                    HoldMode::HoldAndToggle => {
                        if key_ref.is_modifier() && self.modifier_map[&key_ref.get_key_name()] == 1
                        {
                            self.modifier_map.insert(key_ref.get_key_name(), 2);
                            None
                        } else if key_ref.is_modifier()
                            && self.modifier_map[&key_ref.get_key_name()] == 2
                        {
                            self.modifier_map.insert(key_ref.get_key_name(), 0);

                            Some(KeyState::RELEASED)
                        } else {
                            if key_ref.is_modifier() {
                                self.modifier_map.insert(key_ref.get_key_name(), 1);
                            }

                            Some(KeyState::PRESSED)
                        }
                    }
                };

                if let Some(c) = command {
                    self.device.write(&[KeyEvent::new(key, c).into()])?;
                }

                if matches!(
                    conf_g.hold_mode,
                    HoldMode::Hold | HoldMode::Toggle | HoldMode::HoldAndToggle
                ) && key_ref.is_modifier()
                {
                    self.refresh_ui();
                }

                Ok(())
            }
            KeyWrap::Custom(custom) => match custom {
                CustomKey::Fn => {
                    self.logger.trace("PRESSED:Fn");

                    self.is_fn = match conf_g.hold_mode {
                        HoldMode::None => true,
                        HoldMode::Hold | HoldMode::Toggle => {
                            if self.modifier_map["Fn"] == 1 {
                                self.modifier_map.insert("Fn".to_string(), 0);
                                false
                            } else {
                                self.modifier_map.insert("Fn".to_string(), 1);
                                true
                            }
                        }
                        HoldMode::HoldAndToggle => {
                            if self.modifier_map["Fn"] == 1 {
                                self.modifier_map.insert("Fn".to_string(), 2);
                                true
                            } else if self.modifier_map["Fn"] == 2 {
                                self.modifier_map.insert("Fn".to_string(), 0);
                                false
                            } else {
                                self.modifier_map.insert("Fn".to_string(), 1);
                                true
                            }
                        }
                    };

                    self.refresh_ui();

                    Ok(())
                }
                _ => panic!(),
            },
        }
    }

    pub fn key_up(&mut self, key_addr: (usize, usize)) -> std::io::Result<()> {
        let is_g = self.input_state.read().unwrap();
        let kb_g = is_g.get_kb_ref();
        let conf_g = is_g.get_conf_ref();

        let key_ref = kb_g.get_keydef_by_addr(key_addr);

        match key_ref.key_code(self.is_fn) {
            KeyWrap::Default(key) => {
                if matches!(conf_g.hold_mode, HoldMode::None)
                    || matches!(
                        conf_g.hold_mode,
                        HoldMode::Hold | HoldMode::Toggle | HoldMode::HoldAndToggle
                    ) && !key_ref.is_modifier()
                {
                    self.logger.trace(format!("RELEASED:{:?}", key));

                    self.device
                        .write(&[KeyEvent::new(key, KeyState::RELEASED).into()])?;

                    if matches!(conf_g.hold_mode, HoldMode::Hold | HoldMode::HoldAndToggle) {
                        let mut names = vec![];

                        for (name, flg) in self.modifier_map.iter_mut() {
                            if flg == &1 {
                                *flg = 0;
                                names.push(name);
                            }
                        }

                        for mod_key_def in kb_g.get_keydefs_by_names(names) {
                            if let KeyWrap::Default(mod_key) = mod_key_def.key_code(self.is_fn) {
                                self.device
                                    .write(&[KeyEvent::new(mod_key, KeyState::RELEASED).into()])?;
                            }
                        }

                        self.is_fn = false;
                        self.refresh_ui();
                    }

                    if matches!(
                        conf_g.hold_mode,
                        HoldMode::Hold | HoldMode::Toggle | HoldMode::HoldAndToggle
                    ) && key_ref.is_modifier()
                    {
                        self.refresh_ui();
                    }
                }

                Ok(())
            }
            KeyWrap::Custom(custom) => match custom {
                CustomKey::Fn => {
                    if matches!(conf_g.hold_mode, HoldMode::None) {
                        self.logger.trace("RELEASED:Fn");
                        self.is_fn = false;

                        self.refresh_ui();
                    }

                    Ok(())
                }
                _ => panic!(),
            },
        }
    }

    pub fn refresh_ui(&self) {
        let is_g = self.input_state.read().unwrap();
        let kb_g = is_g.get_kb_ref();
        let conf_g = is_g.get_conf_ref();

        for (i, r) in kb_g.rows.iter().enumerate() {
            for (j, c) in r.keys.iter().enumerate() {
                if let KeyDef::Multi { .. } = c {
                    if self.is_fn {
                        let _ = self.ui_eve_sender.send(UiEvent::SetKeyText {
                            pos: (i, j),
                            texts: (
                                c.label(false, true).to_string(),
                                c.label(true, true).to_string(),
                                c.label(false, false).to_string(),
                            ),
                        });
                    } else {
                        let _ = self.ui_eve_sender.send(UiEvent::SetKeyText {
                            pos: (i, j),
                            texts: (
                                c.label(false, false).to_string(),
                                c.label(true, false).to_string(),
                                c.label(false, true).to_string(),
                            ),
                        });
                    }
                }

                if !matches!(conf_g.hold_mode, HoldMode::None) && c.is_modifier() {
                    if self.modifier_map[&c.get_key_name()] >= 1 {
                        let (rmv_name, add_name) = {
                            if matches!(conf_g.hold_mode, HoldMode::Hold)
                                || matches!(conf_g.hold_mode, HoldMode::HoldAndToggle)
                                    && self.modifier_map[&c.get_key_name()] == 1
                            {
                                self.logger.trace(format!(
                                    "{}: COLOER_CHENGE_TO_HOLDED",
                                    c.get_key_name()
                                ));

                                ("toggled-key".to_string(), "holded-key".to_string())
                            } else {
                                self.logger.trace(format!(
                                    "{}: COLOER_CHENGE_TO_TOGGLED",
                                    c.get_key_name()
                                ));

                                ("holded-key".to_string(), "toggled-key".to_string())
                            }
                        };

                        let _ = self.ui_eve_sender.send(UiEvent::CtlKeyStyle {
                            pos: (i, j),
                            mode: StyleCtl::Rmv,
                            name: rmv_name,
                        });

                        let _ = self.ui_eve_sender.send(UiEvent::CtlKeyStyle {
                            pos: (i, j),
                            mode: StyleCtl::Add,
                            name: add_name,
                        });
                    } else {
                        self.logger
                            .trace(format!("{}: COLOER_CHENGE_TO_DEFAULT", c.get_key_name()));

                        let _ = self.ui_eve_sender.send(UiEvent::CtlKeyStyle {
                            pos: (i, j),
                            mode: StyleCtl::Rmv,
                            name: "holded-key".to_string(),
                        });
                        let _ = self.ui_eve_sender.send(UiEvent::CtlKeyStyle {
                            pos: (i, j),
                            mode: StyleCtl::Rmv,
                            name: "toggled-key".to_string(),
                        });
                    }
                }
            }
        }
    }
}
