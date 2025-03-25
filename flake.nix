{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.nightly.latest.default;

        commonBuildInputs = with pkgs; [
            # Rust toolchain with wasm target
            (rustVersion.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            
            binaryen
            # WASM tools
            wasm-bindgen-cli
            
            # Leptos specific tools
            leptosfmt
            cargo-leptos
            cargo-generate
            
            # Additional development tools
            dart-sass
            pkg-config
            openssl
            nodePackages.npm
          ];

        app = pkgs.rustPlatform.buildRustPackage {
          pname = "hours_counter";
          version = "0.1.0";

          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;

          doCheck = false;

          nativeBuildInputs = commonBuildInputs;

          # buildInputs = commonBuildInputs;

          HOME = pkgs.lib.cleanSource ./.;

          buildPhase = ''
            cargo leptos build --release
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/hours_counter $out/bin/
            mkdir -p $out/share/hours_counter
            cp -r target/site/* $out/share/hours_counter/
          '';
        };
      in
      {
        packages = {
          inherit app;
          default = app;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = commonBuildInputs;

          shellHook = ''
            echo "Rust development environment loaded"
            cargo --version
          '';
        };
      }
    );
}
