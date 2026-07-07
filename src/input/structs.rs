use evdevil::{
    event::{Key, KeyEvent, KeyState},
    uinput::UinputDevice,
};

#[derive(Debug, Clone, Copy)]
pub enum InputCommand {
    KeyDown(Key),
    KeyUp(Key),
    Shutdown,
}

pub struct InputSender {
    device: UinputDevice,
}

impl InputSender {
    pub fn new(keys: Vec<Key>) -> anyhow::Result<Self> {
        let device = UinputDevice::builder()?
            .with_keys(keys)?
            .build("Screen Keyboard")?;

        Ok(Self { device })
    }

    pub fn key_down(&self, key: Key) -> std::io::Result<()> {
        self.device
            .write(&[KeyEvent::new(key, KeyState::PRESSED).into()])
    }

    pub fn key_up(&self, key: Key) -> std::io::Result<()> {
        self.device
            .write(&[KeyEvent::new(key, KeyState::RELEASED).into()])
    }
}
