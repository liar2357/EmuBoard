[English](README.md)

---

# 概要

EmuBoardはLinux/Wayland環境向けのスクリーンキーボードです。
uinputによる仮想キーボードを使用する仕組みにより既存のIMEと共存・併用することが可能です。

> **_本プロジェクトはver1.0に達していません。_**
> **_不安定な動作や破壊的変更の可能性があります。_**

# 技術スタック

- 言語: Rust
- GUI FW: GTK4（gtk-rs）

# 対応環境

Wayland環境及びGTK Layer-ShellをサポートするWaylandコンポジターが必要です。
ex) Hyprland, Niri

## 動作確認済み環境

| Distribution | DE/WM           |
| ------------ | --------------- |
| NixOS 26.11  | Hyprland 0.56.2 |

その他の環境については今後サポートを広げていく予定です。

# インストール

現時点ではNix（flake）でのみサポートをしています。
flake.nixでURLを指定し、パッケージ定義からインストールください。

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

参考: [実際の使用例](https://github.com/liar2357/Dotfiles/blob/main/flake.nix)

その他の環境の場合はお手数ですがリポジトリのクローンとセルフビルドを手動でお願いします。
別途以下をご参照ください。

- [package.nix(Nix向けパッケージ定義)](./package.nix)
- [rust.nix(Rust開発用依存パッケージ)](https://github.com/liar2357/nix-dev-common/blob/main/rust.nix)
- [gtk.nix(GTK開発用依存パッケージ)](https://github.com/liar2357/nix-dev-common/blob/main/gtk.nix)

# 使い方

本プロジェクトには`emu-board`と`emu-boardctl`の2つのバイナリが含まれています。

- emu-board: スクリーンキーボード本体
- emu-boardctl: UIの表示等を操作するCLIツール

## emu-board

`emu-board`コマンドで実行するとキーボードのUIを持つアプリが立ち上がります。
標準では物理キーボードを再現した挙動で操作できます。
設定ファイル(`$HOME/.config/emu-board/config.toml`)で細かな挙動を制御できます。
項目ごとに設定がされていない場合はデフォルトの設定にフォールバックします。

> EmuBoardはLinuxの`uinput`を使用するため、`/dev/uinput`へのアクセス権限が必要です。
> ディストリビューションによって設定方法が異なります。

### 設定項目一覧

| 項目             | 説明             | デフォルト   |
| ---------------- | ---------------- | ------------ |
| layout           | キーボード配列   | "JIS-QWERTY" |
| hold_mode        | 修飾キーの動作   | "None"       |
| default_monitor  | 表示するモニター | "auto"       |
| default_ui_view  | 起動時のUI表示   | true         |
| default_ui_place | UIの表示位置     | "Lower"      |

### 設定ファイル記述例

```toml
# $HOME/.config/emu-board/config.toml

# layout
# "JIS-QWERTY" -> 日本語QWERTY配列
# "US-QWERTY" -> US英語QWERTY配列
layout="JIS-QWERTY"

# hold_mode
# "None" -> 修飾キーのホールドを行いません。
# "Hold" -> 修飾キーを押すとホールドされ修飾キー以外のキーが押されるとリリースされます。
# "Toggle" -> 修飾キーが押されるごとに有効/無効が切替わります。
hold_mode="Hold"

# default_monitor
# "auto" -> 接続されているモニターのうち接続名が辞書順で最も若いものを自動で選択します。
# "<任意の接続名>" -> そのモニターに表示します。
default_monitor="eDP-1"

# default_ui_view
# true -> 起動直後からUIが表示されます。
# false -> 起動直後はUIが表示されません。
default_ui_view=false


# default_ui_place
# "Lower" -> 画面下部にUIを表示します。
# "Upper" -> 画面上部にUIを表示します。
default_ui_place="Lower"
```

## emu-boardctl

### コマンド一覧

| 名前            | 説明                             |
| --------------- | -------------------------------- |
| toggle_ui_view  | UIの表示/非表示を切り替えます    |
| show_ui_view    | UIを表示します                   |
| hide_ui_view    | UIを非表示にします。             |
| toggle_ui_place | UIの表示位置を上下で切り替えます |
| upper_ui_place  | UIの表示位置を上側にします       |
| lower_ui_place  | UIの表示位置を下側にします       |
| shutdown_app    | アプリを終了します               |

### 使用例

```bash
# UIを表示
emu-boardctl show_ui_view

# UIを上側に移動
emu-boardctl upper_ui_place

# アプリを終了
emu-boardctl shutdown_app
```

# ライセンス

[MIT License](./LICENCE)
