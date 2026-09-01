use crate::ui::structs::StyleCtl;

pub enum ReloadEvent {
    ChangeConfigFile,
    ChangeMonitorWidth,
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
