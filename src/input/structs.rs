use evdevil::{
    event::{KeyEvent, KeyState},
    uinput::UinputDevice,
};

use crate::ui::structs::{CustomKey, KeyDef, KeyWrap, Keyboard, UiEvent};

use std::sync::{Arc, mpsc::Sender};

#[derive(Debug, Clone, Copy)]
pub enum InputCommand {
    KeyDown((usize, usize)),
    KeyUp((usize, usize)),
}

pub struct InputSender {
    device: UinputDevice,
    kb: Arc<Keyboard>,
    ui_eve_sender: Sender<UiEvent>,

    is_fn: bool,
}

impl InputSender {
    pub fn new(kb: Arc<Keyboard>, ui_eve_sender: Sender<UiEvent>) -> anyhow::Result<Self> {
        let device = UinputDevice::builder()?
            .with_keys(kb.supperted_keys())?
            .build(env!("CARGO_PKG_NAME"))?;

        Ok(Self {
            device,
            kb,
            ui_eve_sender,
            is_fn: false,
        })
    }

    pub fn key_down(&mut self, key_addr: (usize, usize)) -> std::io::Result<()> {
        let key_ref = self.kb.get_keydef_by_addr(key_addr);
        match key_ref.key_code(self.is_fn) {
            KeyWrap::Default(key) => {
                eprintln!("PRESSED:{:?}", key);

                self.device
                    .write(&[KeyEvent::new(key, KeyState::PRESSED).into()])
            }
            KeyWrap::Custom(custom) => match custom {
                CustomKey::Fn => {
                    eprintln!("PRESSED:Fn");
                    self.is_fn = true;

                    self.refresh_ui();

                    Ok(())
                }
                _ => panic!(),
            },
        }
    }

    pub fn key_up(&mut self, key_addr: (usize, usize)) -> std::io::Result<()> {
        match self.kb.get_keydef_by_addr(key_addr).key_code(self.is_fn) {
            KeyWrap::Default(key) => {
                eprintln!("RELEASED:{:?}", key);

                self.device
                    .write(&[KeyEvent::new(key, KeyState::RELEASED).into()])
            }
            KeyWrap::Custom(custom) => match custom {
                CustomKey::Fn => {
                    println!("RELEASED:Fn");
                    self.is_fn = false;

                    self.refresh_ui();

                    Ok(())
                }
                _ => panic!(),
            },
        }
    }

    pub fn refresh_ui(&self) {
        for (i, r) in self.kb.rows.iter().enumerate() {
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
            }
        }
    }
}
