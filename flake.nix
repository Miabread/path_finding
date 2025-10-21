{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              # Rust
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
              })

              # Bevy
              udev
              alsa-lib
              vulkan-loader
              libxkbcommon
              wayland
            ];

            nativeBuildInputs = [ pkg-config ];

            LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
            PKG_CONFIG_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
            RUSTFLAGS =
              "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath buildInputs}";
          };
      });
}
