{ pkgs ? import <nixpkgs> {} }:

let
  unstableTarball =
    fetchTarball
      https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
  pkgs = import unstableTarball {};
in
  pkgs.mkShell {
    buildInputs = [
      pkgs.rustc
      pkgs.cargo
      pkgs.gcc
      pkgs.rustfmt
      pkgs.clippy
      pkgs.libnfc
      pkgs.pcsc-tools
      pkgs.pcsclite
      pkgs.rpi-imager
    ];

    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    shellHook = ''
      echo "Welcome to the Rust development environment!"
      echo "Rust version: $(rustc --version)"
      echo "----------------------------------------------------------"
      echo "set -x OPENSSL_DIR ${pkgs.openssl.dev}; set -x PKG_CONFIG_PATH ${pkgs.openssl.dev}/lib/pkgconfig; set -x OPENSSL_NO_VENDOR 1; set -x OPENSSL_LIB_DIR ${pkgs.lib.getLib pkgs.openssl}/lib;" | xclip -selection clipboard
      echo "set env from clipboard"
    '';
  }

