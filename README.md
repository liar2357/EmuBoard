[日本語版](./README.jp.md)

---

# Overview

EmuBoard is an on-screen keyboard for Linux/Wayland environments.
It uses a virtual keyboard based on "uinput", allowing it to coexist and work alongside existing IMEs.

«This project has not reached version 1.0 yet.
It may exhibit unstable behavior and introduce breaking changes.»

# Tech Stack

- Language: Rust
- GUI Framework: GTK4 (gtk-rs)

# Supported Environment

A Wayland environment and a Wayland compositor that supports GTK Layer Shell are required.
e.g. Hyprland, Niri

# Tested Environment

| Distribution | DE/WM           |
| ------------ | --------------- |
| NixOS 26.11  | Hyprland 0.56.2 |

Support for additional environments is planned for the future.

# Installation

Currently, only Nix (flake) is officially supported.
Specify the repository URL in your "flake.nix" and install EmuBoard through your package definition.

flake.nix

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    emu-board.url = "github:liar2357/EmuBoard";
  };

  outputs = { self, nixpkgs, emu-board, ... }:
    {
      nixosConfigurations.my-pc = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";

        modules = [
          ./configuration.nix
        ];
      };
    };
}
```

configuration.nix

```nix
{ pkgs, emu-board, ... }:

{
  environment.systemPackages = [
    emu-board.packages.${pkgs.system}.default
  ];
}
```

See [an actual usage example](https://github.com/liar2357/Dotfiles/blob/main/flake.nix).

For other environments, please clone the repository and build EmuBoard manually from source.
The following files may also be useful:

- [package.nix (Nix package definition)](./package.nix)
- [rust.nix (Rust development dependencies)](https://github.com/liar2357/nix-dev-common/blob/main/rust.nix)
- [gtk.nix (GTK development dependencies)](https://github.com/liar2357/nix-dev-common/blob/main/gtk.nix)

# Usage

This project provides two binaries:

- "emu-board": The main on-screen keyboard application
- "emu-boardctl": A CLI tool for controlling the UI and other application functions

## emu-board

Running the `emu-board` command launches the application with the on-screen keyboard UI.
By default, it behaves like a physical keyboard.

Fine-grained behavior can be configured using the configuration file at "$HOME/.config/emu-board/config.toml".
Any unspecified options fall back to their default values.

«EmuBoard uses Linux "uinput" and therefore requires permission to access "/dev/uinput".
The configuration required to grant access varies depending on the distribution.»

### Configuration Options

| Option             | Description                              | Default      |
| ------------------ | ---------------------------------------- | ------------ |
| "layout"           | Keyboard layout                          | "JIS-QWERTY" |
| "hold_mode"        | Modifier key behavior                    | "None"       |
| "default_monitor"  | Monitor on which to display the keyboard | "auto"       |
| "default_ui_view"  | Whether to show the UI at startup        | true         |
| "default_ui_place" | Position of the UI                       | "Lower"      |

### Example Configuration

```toml

# $HOME/.config/emu-board/config.toml

# layout
# "JIS-QWERTY" -> Japanese QWERTY layout
# "US-QWERTY" -> US English QWERTY layout

layout="JIS-QWERTY"

# hold_mode
# "None" -> Modifier keys are not held.
# "Hold" -> A modifier key is held until a non-modifier key is pressed.
# "Toggle" -> The modifier key is toggled on/off each time it is pressed.

hold_mode="Hold"

# default_monitor
# "auto" -> Automatically selects the connected monitor with the lexicographically smallest connector name.
# "<any connector name>" -> Displays the UI on the specified monitor.

default_monitor="eDP-1"

# default_ui_view
# true -> The UI is displayed immediately after startup.
# false -> The UI is hidden immediately after startup.

default_ui_view=false

# default_ui_place
# "Lower" -> Displays the UI at the bottom of the screen.
# "Upper" -> Displays the UI at the top of the screen.

default_ui_place="Lower"
```

## emu-boardctl

### Commands

| Command         | Description                                    |
| --------------- | ---------------------------------------------- |
| toggle_ui_view  | Toggle the UI visibility                       |
| show_ui_view    | Show the UI                                    |
| hide_ui_view    | Hide the UI                                    |
| toggle_ui_place | Toggle the UI position between upper and lower |
| upper_ui_place  | Move the UI to the upper position              |
| lower_ui_place  | Move the UI to the lower position              |
| shutdown_app    | Shut down the application                      |

### Examples

```bash
# Show the UI
emu-boardctl show_ui_view

# Move the UI to the upper position
emu-boardctl upper_ui_place

# Shut down the application
emu-boardctl shutdown_app
```

# License

[MIT License](./LICENCE)
