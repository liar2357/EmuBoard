use evdevil::event::Key;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum LogicalKey {
    // 入力系
    Enter,
    Backspace,
    Tab,
    Space,

    // 修飾キー(L)
    LShift,
    LCtrl,
    LAlt,
    LSuper,

    // 修飾キー(R)
    RShift,
    RCtrl,
    RAlt,
    RSuper,

    // トグル系
    CapsLock,

    // システム/制御系
    Escape,
    Insert,
    Delete,
    Pause,
    PrintScreen,

    // ナビゲーション系
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,

    // ファンクションキー
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // JIS
    ZenkakuHankaku,
    Muhenkan,
    Henkan,
    Katakana,

    // 例外
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub rows: Vec<KeyLine>,
}

#[derive(Debug, Deserialize)]
pub struct KeyLine {
    pub keys: Vec<KeyDef>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum KeyDef {
    #[serde(rename = "char")]
    Char {
        label: String,
        shifted: Option<String>,
        keycode: String,

        #[serde(default = "default_width")]
        width: f32,

        #[serde(default = "default_height")]
        height: f32,
    },

    #[serde(rename = "special")]
    Special {
        label: String,
        logical: LogicalKey,

        #[serde(default = "default_width")]
        width: f32,

        #[serde(default = "default_height")]
        height: f32,
    },
}
impl KeyDef {
    pub fn label(&self, shift: bool) -> &str {
        match self {
            KeyDef::Char { label, shifted, .. } => {
                if shift {
                    shifted.as_deref().unwrap_or(label)
                } else {
                    label
                }
            }

            KeyDef::Special { label, .. } => {
                if shift {
                    ""
                } else {
                    label
                }
            }
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            KeyDef::Char { width, .. } => *width,
            KeyDef::Special { width, .. } => *width,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            KeyDef::Char { height, .. } => *height,
            KeyDef::Special { height, .. } => *height,
        }
    }

    pub fn key_code(&self) -> Key {
        match self {
            KeyDef::Char { keycode, .. } => match keycode.as_str() {
                "1" => Key::KEY_1,
                "2" => Key::KEY_2,
                "3" => Key::KEY_3,
                "4" => Key::KEY_4,
                "5" => Key::KEY_5,
                "6" => Key::KEY_6,
                "7" => Key::KEY_7,
                "8" => Key::KEY_8,
                "9" => Key::KEY_9,
                "0" => Key::KEY_0,

                "A" => Key::KEY_A,
                "B" => Key::KEY_B,
                "C" => Key::KEY_C,
                "D" => Key::KEY_D,
                "E" => Key::KEY_E,
                "F" => Key::KEY_F,
                "G" => Key::KEY_G,
                "H" => Key::KEY_H,
                "I" => Key::KEY_I,
                "J" => Key::KEY_J,
                "K" => Key::KEY_K,
                "L" => Key::KEY_L,
                "M" => Key::KEY_M,
                "N" => Key::KEY_N,
                "O" => Key::KEY_O,
                "P" => Key::KEY_P,
                "Q" => Key::KEY_Q,
                "R" => Key::KEY_R,
                "S" => Key::KEY_S,
                "T" => Key::KEY_T,
                "U" => Key::KEY_U,
                "V" => Key::KEY_V,
                "W" => Key::KEY_W,
                "X" => Key::KEY_X,
                "Y" => Key::KEY_Y,
                "Z" => Key::KEY_Z,

                "MINUS" => Key::KEY_MINUS,
                "EQUAL" => Key::KEY_EQUAL,
                "YEN" => Key::KEY_YEN,
                "LEFTBRACE" => Key::KEY_LEFTBRACE,
                "RIGHTBRACE" => Key::KEY_RIGHTBRACE,
                "SEMICOLON" => Key::KEY_SEMICOLON,
                "APOSTROPHE" => Key::KEY_APOSTROPHE,
                "BACKSLASH" => Key::KEY_BACKSLASH,
                "COMMA" => Key::KEY_COMMA,
                "DOT" => Key::KEY_DOT,
                "SLASH" => Key::KEY_SLASH,
                "RO" => Key::KEY_RO,

                _ => panic!(),
            },
            KeyDef::Special { logical, .. } => match logical {
                LogicalKey::Enter => Key::KEY_ENTER,
                LogicalKey::Backspace => Key::KEY_BACKSPACE,
                LogicalKey::Tab => Key::KEY_TAB,
                LogicalKey::Space => Key::KEY_SPACE,
                LogicalKey::LShift => Key::KEY_LEFTSHIFT,
                LogicalKey::LCtrl => Key::KEY_LEFTCTRL,
                LogicalKey::LAlt => Key::KEY_LEFTALT,
                LogicalKey::LSuper => Key::KEY_LEFTMETA,
                LogicalKey::RShift => Key::KEY_RIGHTSHIFT,
                LogicalKey::RCtrl => Key::KEY_RIGHTCTRL,
                LogicalKey::RAlt => Key::KEY_RIGHTALT,
                LogicalKey::RSuper => Key::KEY_RIGHTMETA,
                LogicalKey::CapsLock => Key::KEY_CAPSLOCK,
                LogicalKey::Escape => Key::KEY_ESC,
                LogicalKey::Insert => Key::KEY_INSERT,
                LogicalKey::Delete => Key::KEY_DELETE,
                LogicalKey::Pause => Key::KEY_PAUSE,
                LogicalKey::PrintScreen => Key::KEY_SYSRQ,
                LogicalKey::ArrowUp => Key::KEY_UP,
                LogicalKey::ArrowDown => Key::KEY_DOWN,
                LogicalKey::ArrowLeft => Key::KEY_LEFT,
                LogicalKey::ArrowRight => Key::KEY_RIGHT,
                LogicalKey::Home => Key::KEY_HOME,
                LogicalKey::End => Key::KEY_END,
                LogicalKey::PageUp => Key::KEY_PAGEUP,
                LogicalKey::PageDown => Key::KEY_PAGEDOWN,
                LogicalKey::F1 => Key::KEY_F1,
                LogicalKey::F2 => Key::KEY_F2,
                LogicalKey::F3 => Key::KEY_F3,
                LogicalKey::F4 => Key::KEY_F4,
                LogicalKey::F5 => Key::KEY_F5,
                LogicalKey::F6 => Key::KEY_F6,
                LogicalKey::F7 => Key::KEY_F7,
                LogicalKey::F8 => Key::KEY_F8,
                LogicalKey::F9 => Key::KEY_F9,
                LogicalKey::F10 => Key::KEY_F10,
                LogicalKey::F11 => Key::KEY_F11,
                LogicalKey::F12 => Key::KEY_F12,
                LogicalKey::ZenkakuHankaku => Key::KEY_ZENKAKUHANKAKU,
                LogicalKey::Muhenkan => Key::KEY_MUHENKAN,
                LogicalKey::Henkan => Key::KEY_HENKAN,
                LogicalKey::Katakana => Key::KEY_KATAKANA,
                _ => panic!(),
            },
        }
    }
}

fn default_width() -> f32 {
    1.0
}

fn default_height() -> f32 {
    1.0
}
