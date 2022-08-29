#!/usr/bin/env bash
set -euxo pipefail

nix build .#unremarkableNotes

# We copy the binary out of our nix store to make it writable in order
# to use patchelf to set the correct ELF interpreter path, see
# https://nixos.wiki/wiki/Packaging/Binaries#The_Dynamic_Loader
# TODO: this should be done in unremarkableNotes installHook.
cp ./result/bin/unremarkable-notes unremarkable
chmod +w unremarkable
patchelf --set-interpreter /lib/ld-linux-armhf.so.3 unremarkable


scp unremarkable remarkable:
ssh remarkable "./unremarkable"
