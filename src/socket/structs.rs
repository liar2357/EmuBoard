use std::{
    fmt::{Display, Formatter, Result as fResult},
    str::FromStr,
};

#[derive(Debug)]
pub enum SocketCommand {
    ToggleUiView,
    ShowUiView,
    HideUiView,
    ToggleUiPlace,
    UpperUiPlace,
    LowerUiPlace,
    ShutdownApp,
}

impl SocketCommand {
    pub fn print_all() {
        let arr = vec![
            Self::ToggleUiView,
            Self::ShowUiView,
            Self::HideUiView,
            Self::ToggleUiPlace,
            Self::UpperUiPlace,
            Self::LowerUiPlace,
            Self::ShutdownApp,
        ];

        eprintln!("Available Commands");
        eprintln!("--------------------");

        for a in arr {
            eprintln!("{}", a);
        }
    }
}

impl Display for SocketCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        let s = match self {
            Self::ToggleUiView => "toggle_ui_view",
            Self::ShowUiView => "show_ui_view",
            Self::HideUiView => "hide_ui_view",
            Self::ToggleUiPlace => "toggle_ui_place",
            Self::UpperUiPlace => "upper_ui_place",
            Self::LowerUiPlace => "lower_ui_place",
            Self::ShutdownApp => "shutdown_app",
        };

        write!(f, "{s}")
    }
}

impl FromStr for SocketCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle_ui_view" => Ok(Self::ToggleUiView),
            "show_ui_view" => Ok(Self::ShowUiView),
            "hide_ui_view" => Ok(Self::HideUiView),
            "toggle_ui_place" => Ok(Self::ToggleUiPlace),
            "upper_ui_place" => Ok(Self::UpperUiPlace),
            "lower_ui_place" => Ok(Self::LowerUiPlace),
            "shutdown_app" => Ok(Self::ShutdownApp),
            _ => Err(()),
        }
    }
}
