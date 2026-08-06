{
  lib,
  rustPlatform,

  pkg-config,
  wrapGAppsHook4,

  gtk4,
  gtk4-layer-shell,
  glib,
}:

rustPlatform.buildRustPackage {

  pname = "emu-board";
  version = "0.9.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    gtk4
    gtk4-layer-shell
    glib
  ];

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin

    install -Dm755 \
      target/release/emu-board \
      $out/bin/emu-board

    install -Dm755 \
      target/release/emu-boardctl \
      $out/bin/emu-boardctl

    mkdir -p \
      $out/share/applications

    install -Dm644 \
      data/io.github.liar2357.emu-board.desktop \
      $out/share/applications/io.github.liar2357.emu-board.desktop

    mkdir -p \
      $out/share/icons/hicolor/scalable/apps

    install -Dm644 \
      data/icons/scalable/io.github.liar2357.emu-board.svg \
      $out/share/icons/hicolor/scalable/apps/io.github.liar2357.emu-board.svg

    runHook postInstall
  '';

  meta = with lib; {
    description = "Wayland screen keyboard";
    homepage = "https://github.com/liar2357/emu-board";
    license = licenses.mit;
    platforms = platforms.linux;
    mainProgram = "emu-board";
  };
}
