{
  description = "Rust dev shell template";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # or nightly
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };

        # For existing toolchain.
        # toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        # help at https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            packages = [
              toolchain
            ];
            buildInputs = [
            ];
            nativeBuildInputs = [
              pkg-config
            ];

            # https://github.com/NixOS/nixpkgs/issues/177952#issuecomment-3172381779
            NIX_NO_SELF_RPATH = true;
          };
      }
    );
}
