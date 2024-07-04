{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            wasm-pack = prev.rustPlatform.buildRustPackage rec {
              pname = "wasm-pack";
              version = "0.13.0";

              src = prev.fetchFromGitHub {
                owner = "rustwasm";
                repo = "wasm-pack";
                rev = "refs/tags/v${version}";
                hash = "sha256-NEujk4ZPQ2xHWBCVjBCD7H6f58P4KrwCNoDHKa0d5JE=";
              };

              cargoHash = "sha256-pFKGQcWW1/GaIIWMyWBzts4w1hMu27hTG/uUMjkfDMo=";
              nativeBuildInputs = with prev; [cmake];

              buildInputs = prev.lib.optional prev.stdenv.isDarwin prev.darwin.apple_sdk.frameworks.Security;

              # Most tests rely on external resources and build artifacts.
              # Disabling check here to work with build sandboxing.
              doCheck = false;

              propagatedBuildInputs = with pkgs; [nodejs_22];
            };
          })
        ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default;
      in rec {
        # For `nix develop`:
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          nativeBuildInputs = with pkgs; [
            (rust.override {
              extensions = ["rust-src"];
              targets = ["wasm32-unknown-unknown"];
            })
            rust-analyzer
            wasm-pack
          ];
        };
      }
    );
}
