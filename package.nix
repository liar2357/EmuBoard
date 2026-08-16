{
  lib,
  stdenv,
  rustPlatform,

  pkg-config,
  wrapGAppsHook4,

  gtk4,
  gtk4-layer-shell,
  glib,
}:

rustPlatform.buildRustPackage {

  pname = "emu-board";
  version = "0.1.0";

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

      echo "=== target ==="
      find target -maxdepth 3 -type f -executable -print || true

      echo "=== cargo target ==="
      find . -maxdepth 4 -type f -name 'emu-board*' -print || true

     install -Dm755 \
      target/${stdenv.hostPlatform.rust.rustcTarget}/release/emu-board \
      $out/bin/emu-board

    install -Dm755 \
      target/${stdenv.hostPlatform.rust.rustcTarget}/release/emu-boardctl \
      $out/bin/emu-boardctl

    install -Dm644 \
      data/emu-board.desktop \
      $out/share/applications/emu-board.desktop

    install -Dm644 \
      data/icons/64x64/emu-board.png \
      $out/share/icons/hicolor/64x64/apps/emu-board.png

    install -Dm644 \
      data/icons/128x128/emu-board.png \
      $out/share/icons/hicolor/128x128/apps/emu-board.png
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
