{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.nightly.latest.default;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with wasm target
            (rustVersion.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            
            # WASM tools
            wasm-bindgen-cli
            
            # Leptos specific tools
            leptosfmt
            cargo-leptos
            cargo-generate
            
            # Additional development tools
            sass
            dart-sass
            pkg-config
            openssl
            nodePackages.npm
          ];

          shellHook = ''
            echo "Rust development environment loaded"
            cargo --version
          '';
        };
      }
    );
}
