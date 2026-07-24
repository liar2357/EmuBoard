use evdevil::event::Key;
use gtk::prelude::WidgetExt;
use serde::Deserialize;
use std::{cmp::max, collections::HashSet};

#[derive(Debug, Clone, Copy)]
pub enum KeyWrap {
    Default(Key),
    Custom(CustomKey),
}

#[derive(Debug, Deserialize)]
pub enum LogicalKey {
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
    Hiragana,

    // 例外
    Other,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum CustomKey {
    Fn,
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub rows: Vec<KeyLine>,
}
impl Keyboard {
    pub fn supperted_keys(&self) -> Vec<Key> {
        let mut set: HashSet<Key> = HashSet::new();

        for row in &self.rows {
            for key in &row.keys {
                set.extend(key.key_patterns());
            }
        }

        set.into_iter().collect()
    }

    pub fn calc_key_unit_in_line(&self) -> i32 {
        let mut unit_in_line = 0;
        for line in self.rows.iter() {
            unit_in_line = max(
                unit_in_line,
                line.keys.iter().map(|v| v.width()).sum::<i32>(),
            );
        }
        unit_in_line
    }

    pub fn get_keydef_by_addr(&self, (r, c): (usize, usize)) -> &KeyDef {
        &self.rows[r].keys[c]
    }

    pub fn get_keydefs_by_names(&self, names: Vec<&String>) -> Vec<&KeyDef> {
        let mut work = vec![];

        for line in self.rows.iter() {
            for def in line.keys.iter() {
                if names.contains(&&def.get_key_name()) {
                    work.push(def);
                }
            }
        }

        work
    }
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
        width: i32,

        #[serde(default = "default_height")]
        height: i32,
    },

    #[serde(rename = "special")]
    Special {
        label: String,
        logical: LogicalKey,

        #[serde(default = "default_width")]
        width: i32,

        #[serde(default = "default_height")]
        height: i32,
    },

    #[serde(rename = "custom")]
    Custom {
        label: String,
        custom: CustomKey,

        #[serde(default = "default_width")]
        width: i32,

        #[serde(default = "default_height")]
        height: i32,
    },

    #[serde(rename = "multi")]
    Multi { key: Vec<KeyDef> },
}
impl KeyDef {
    pub fn label(&self, shift: bool, is_fn: bool) -> &str {
        match self {
            KeyDef::Char { label, shifted, .. } => {
                if is_fn {
                    ""
                } else if shift {
                    shifted.as_deref().unwrap_or(label)
                } else {
                    label
                }
            }

            KeyDef::Special { label, .. } => {
                if is_fn || shift {
                    ""
                } else {
                    label
                }
            }

            KeyDef::Custom { label, .. } => {
                if is_fn || shift {
                    ""
                } else {
                    label
                }
            }

            KeyDef::Multi { key } => {
                if !is_fn {
                    key[0].label(shift, is_fn)
                } else {
                    key[1].label(shift, false)
                }
            }
        }
    }

    pub fn width(&self) -> i32 {
        match self {
            KeyDef::Char { width, .. } => *width,
            KeyDef::Special { width, .. } => *width,
            KeyDef::Custom { width, .. } => *width,
            KeyDef::Multi { key } => key[0].width(),
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            KeyDef::Char { height, .. } => *height,
            KeyDef::Special { height, .. } => *height,
            KeyDef::Custom { height, .. } => *height,
            KeyDef::Multi { key } => key[0].height(),
        }
    }

    pub fn key_code(&self, is_fn: bool) -> KeyWrap {
        match self {
            KeyDef::Char { keycode, .. } => KeyWrap::Default(match keycode.as_str() {
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
            }),
            KeyDef::Special { logical, .. } => KeyWrap::Default(match logical {
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
                LogicalKey::Hiragana => Key::KEY_HIRAGANA,
                _ => panic!(),
            }),
            KeyDef::Custom { custom, .. } => KeyWrap::Custom(*custom),
            KeyDef::Multi { key } => {
                if !is_fn {
                    key[0].key_code(is_fn)
                } else {
                    key[1].key_code(is_fn)
                }
            }
        }
    }

    pub fn key_patterns(&self) -> HashSet<Key> {
        let mut set = HashSet::new();

        match self {
            KeyDef::Char { .. } | KeyDef::Special { .. } => {
                if let KeyWrap::Default(k) = self.key_code(false) {
                    set.insert(k);
                }
            }

            KeyDef::Multi { key } => {
                for k in key {
                    if let KeyWrap::Default(k) = k.key_code(false) {
                        set.insert(k);
                    }
                }
            }

            _ => {}
        }

        set
    }

    pub fn is_modifier(&self) -> bool {
        match self {
            KeyDef::Char { .. } | KeyDef::Multi { .. } => false,
            KeyDef::Special { logical, .. } => matches!(
                logical,
                LogicalKey::LAlt
                    | LogicalKey::RAlt
                    | LogicalKey::LCtrl
                    | LogicalKey::RCtrl
                    | LogicalKey::LShift
                    | LogicalKey::RShift
                    | LogicalKey::LSuper
                    | LogicalKey::RSuper
            ),

            KeyDef::Custom { custom, .. } => matches!(custom, CustomKey::Fn),
        }
    }

    pub fn get_key_name(&self) -> String {
        match self {
            KeyDef::Char { keycode, .. } => keycode.clone(),
            KeyDef::Special { logical, .. } => format!("{:?}", logical),
            KeyDef::Custom { custom, .. } => format!("{:?}", custom),
            KeyDef::Multi { key, .. } => format!(
                "multi({:?},{:?})",
                key[0].get_key_name(),
                key[1].get_key_name()
            ),
        }
    }
}

fn default_width() -> i32 {
    1
}

fn default_height() -> i32 {
    1
}

pub struct KeyComponents {
    frame: gtk::Frame,
    nomal: gtk::Label,
    shift: gtk::Label,
    func: gtk::Label,
}
impl KeyComponents {
    pub fn set_text(&mut self, texts: (&str, &str, &str)) {
        self.nomal.set_label(texts.0);
        self.shift.set_label(texts.1);
        self.func.set_label(texts.2);
    }

    pub fn add_css_class(&mut self, class_name: &str) {
        self.frame.add_css_class(class_name);
    }

    pub fn rmv_css_class(&mut self, class_name: &str) {
        self.frame.remove_css_class(class_name);
    }
}

pub struct KeyComponentsTable {
    table: Vec<Vec<KeyComponents>>,
}
impl KeyComponentsTable {
    pub fn new() -> Self {
        Self { table: vec![] }
    }

    pub fn append(
        &mut self,
        addr: (usize, usize),
        obj: (gtk::Frame, gtk::Label, gtk::Label, gtk::Label),
    ) {
        let work = KeyComponents {
            frame: obj.0,
            nomal: obj.1,
            shift: obj.2,
            func: obj.3,
        };

        if addr.1 == 0 {
            self.table.push(vec![work]);
        } else {
            self.table[addr.0].push(work);
        }
    }

    pub fn set_text(&mut self, addr: (usize, usize), texts: (&str, &str, &str)) {
        self.table[addr.0][addr.1].set_text(texts);
    }

    pub fn add_css_class(&mut self, addr: (usize, usize), class_name: &str) {
        self.table[addr.0][addr.1].add_css_class(class_name);
    }

    pub fn rmv_css_class(&mut self, addr: (usize, usize), class_name: &str) {
        self.table[addr.0][addr.1].rmv_css_class(class_name);
    }
}

pub enum StyleCtl {
    Add,
    Rmv,
}

pub enum UiEvent {
    SetKeyText {
        pos: (usize, usize),
        texts: (String, String, String),
    },
    CtlKeyStyle {
        pos: (usize, usize),
        mode: StyleCtl,
        name: String,
    },
}
