with import <nixpkgs> { };

mkShell rec {
  # Rust
  packages = [ rustc cargo gcc rustfmt clippy ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  # Bevy
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ udev alsa-lib vulkan-loader libxkbcommon wayland ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
