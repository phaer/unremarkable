{
  description = "Utilities for the remarkable2";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachSystem
      [ "x86_64-linux" "armv7l-linux"]
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          remarkablePkgs = pkgs.pkgsCross.remarkable2;

          #linesAreRusty = naerskLib.buildPackage lines-are-rusty;

          unremarkableNotes = remarkablePkgs.rustPlatform.buildRustPackage {
            name = "unremarkable-notes";
            src = ./unremarkable-notes;
            #cargoSha256 = pkgs.lib.fakeSha256;
            cargoSha256 = "sha256-To21JCGDviyLZMTE4lh3nStFbHW1dKKylmu5OhL4biE=";

            nativeBuildInputs = with remarkablePkgs; [ pkg-config ];
            buildInputs = with remarkablePkgs; [ openssl ];
          };

        in
          {
            packages = {
              inherit unremarkableNotes;
              hello = pkgs.hello;
            };
            #defaultPackage = linesAreRusty;

            devShells.default = pkgs.mkShell {
              RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              RMAPI_HOST = "https://remarkable.flawed.cloud";

              buildInputs = with pkgs; [
                cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer pkg-config openssl
                cargo-watch
                rmapi
              ];
            };
          });
}
