{
  description = "A Nix flake for Rust OS development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11"; # Use a stable Nixpkgs channel
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable"; # For newer packages like gemini-cli
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      nixpkgs-unstable,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgsUnstable = import nixpkgs-unstable { inherit system; };

        # Specify a recent stable Rust toolchain
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      in
      {
        devShells.default = pkgs.mkShell {
          name = "copper-osdev-shell";

          packages = with pkgs; [
            rustToolchain
            qemu # For emulating the OS
            llvm # Often useful for Rust tooling and introspection
            clang # Required by some Rust crates/tools
            gdb # For debugging the kernel
            grub2 # For creating bootable images (e.g., GRUB multiboot)
            xorriso # For creating ISO images
            cargo-bootimage # From original shell.nix
            pkgsUnstable.gemini-cli # Added gemini-cli from unstable nixpkgs
          ];

          # Set environment variables for cargo and rustc
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          # Optionally, you might want to set target specific variables if cross-compiling
          # E.g., RUSTFLAGS="-C target-cpu=nehalem"

          shellHook = ''
            echo "Entering Rust OS development shell for Copper."
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "QEMU version: $(qemu-system-x86_64 --version | head -n 1)"
            export PATH="$PWD/.cargo/bin:$PATH" # Ensure locally installed cargo tools are in PATH
          '';
        };
      }
    );
}
