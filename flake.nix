{
  description = "Utilities for the remarkable2";
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:numtide/flake-utils";
    lines-are-rusty = {
      flake = false;
      url = "https://github.com/ax3l/lines-are-rusty";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, lines-are-rusty }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        remarkablePkgs = pkgs.pkgsCross.remarkable2;
        toolchain = pkgs.remarkable2-toolchain;

        linesAreRusty = naersk-lib.buildPackage lines-are-rusty;
        unremarkableNotes = naersk-lib.buildPackage ./unremarkable-notes;
      in
        {

          packages = {
            inherit toolchain linesAreRusty unremarkableNotes;
            remarkable-hello = remarkablePkgs.hello;

          };
          defaultPackage = linesAreRusty;

          devShells.default = pkgs.mkShell {
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
            RMAPI_HOST = "https://remarkable.flawed.cloud";


            buildInputs = with pkgs; [
              cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer pkg-config openssl
              rmapi
            ];
         };
        });
    }
