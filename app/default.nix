{ pkgs }:

let
  # Import the rust-overlay
  rustOverlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
    sha256 = "sha256:1ahj99gpajmd1s2zd2bq03yzbhb38pbln5nnv0k3r2b69zwjskw0";
  });

  # Apply the overlay to pkgs
  pkgsWithRustOverlay = pkgs.extend rustOverlay;

  # Select the specific Rust version you want
  rust-1-85-0 = pkgsWithRustOverlay.rust-bin.stable."1.85.0".default.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

in pkgsWithRustOverlay.stdenv.mkDerivation {
  name = "web-app";
  src = ./.;

  # Use the specific Rust version
  nativeBuildInputs = [
    git
    rust-1-85-0
    pkgsWithRustOverlay.trunk
    pkgsWithRustOverlay.wasm-bindgen-cli
    pkgsWithRustOverlay.nodePackages.npm
    pkgsWithRustOverlay.binaryen
    pkgsWithRustOverlay.cacert
  ];

  __noChroot = true;

  # Build phase
  buildPhase = ''
    # Ensure trunk has a home directory
    export HOME=$TMPDIR

    # Build the release version
    cd web_app
    trunk build --release
  '';

  # Install phase - copy the built files to the output directory
  installPhase = ''
    mkdir -p $out
    cp -r dist/* $out/
  '';

  # Add any system dependencies your app needs at runtime
  buildInputs = with pkgs; [ wasm-bindgen-cli ];
}
