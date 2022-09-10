{
  description = "Rust libary to parse files from the remarkable2 eink tablet";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs { inherit system; };
          remarkablePkgs = pkgs.pkgsCross.remarkable2.pkgsStatic;
          unremarkableNotes = remarkablePkgs.rustPlatform.buildRustPackage {
            name = "unremarkable-notes";
            src = builtins.filterSource
              (path: type: type != "directory" || builtins.baseNameOf path != "target")
              (pkgs.lib.cleanSource ./.);
            #cargoSha256 = pkgs.lib.fakeSha256;
            cargoSha256 = "sha256-id1TQeuWyHprwDOYXSrKll8nVxyUXemhjtDOt8oSCUY=";

            nativeBuildInputs = with remarkablePkgs; [ pkg-config ];
            buildInputs = with remarkablePkgs; [ openssl ];
          };

        in
          {
            packages = {
              inherit unremarkableNotes;
            };
            defaultPackage = unremarkableNotes;

            devShells.default = pkgs.mkShell {
              RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              buildInputs = with pkgs; [
                cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer pkg-config openssl
                cargo-watch
              ];
            };
          });
}
