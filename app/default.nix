{ pkgs }:

let
  # Use pkgs.rustPlatform for Rust-specific build tools
  inherit (pkgs) rustPlatform;
in pkgs.stdenv.mkDerivation {
  name = "web-app";
  src = ./.; # Use the current directory as source

  # Add necessary build dependencies
  nativeBuildInputs = with pkgs; [
    trunk
    rustc
    cargo
    wasm-bindgen-cli
    nodePackages.npm
    binaryen # For wasm optimization
  ];

  # Build phase
  buildPhase = ''
    # Ensure trunk has a home directory
    export HOME=$TMPDIR

    # Build the release version
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
