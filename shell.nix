{
  pkgs ? import <nixpkgs> { },
}:
let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
  toolchain = fenix.complete.withComponents [
    "cargo"
    "rustc"
    "rust-src"
    "llvm-tools-preview"
    "clippy"
  ];
in
pkgs.mkShell {
  buildInputs = [
    toolchain
    pkgs.qemu
    pkgs.clang
    pkgs.gdb
    pkgs.cargo-bootimage
  ];
}
