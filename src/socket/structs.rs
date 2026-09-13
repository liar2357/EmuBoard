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
    SwitchProfile(usize),
    ReloadApp,
    ShutdownApp,
}

impl SocketCommand {
    pub fn get_all_comands_string() -> String {
        [
            Self::ToggleUiView,
            Self::ShowUiView,
            Self::HideUiView,
            Self::ToggleUiPlace,
            Self::UpperUiPlace,
            Self::LowerUiPlace,
            Self::SwitchProfile(0),
            Self::ReloadApp,
            Self::ShutdownApp,
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("\n")
        .to_string()
    }

    pub fn print_all() {
        eprintln!("Available Commands");
        eprintln!("------------------");
        eprintln!("{}", Self::get_all_comands_string());
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
            Self::SwitchProfile(_) => "switch_profile <index>",
            Self::ReloadApp => "reload_app",
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
            "reload_app" => Ok(SocketCommand::ReloadApp),
            "shutdown_app" => Ok(Self::ShutdownApp),

            _ => {
                let (command, arg) = s.split_once('-').ok_or(())?;

                match command {
                    "switch_profile" => {
                        let index = arg.parse::<usize>().map_err(|_| ())?;
                        Ok(Self::SwitchProfile(index))
                    }
                    _ => Err(()),
                }
            }
        }
    }
}
