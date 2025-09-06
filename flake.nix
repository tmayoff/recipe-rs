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
        ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.nightly.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl

            nodejs
          ];

          nativeBuildInputs = with pkgs;
            [
              (rust.override {
                extensions = ["rust-src" "rustfmt" "llvm-tools-preview" "rust-analyzer"];
                targets = ["wasm32-unknown-unknown"];
              })

              wasm-pack

              yarn

              cargo-binutils
              cargo-tarpaulin
            ]
            ++ (pkgs.lib.optionals pkgs.stdenv.isLinux [cargo-llvm-cov]);
        };
      }
    );
}
