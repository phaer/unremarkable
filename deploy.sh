#!/usr/bin/env bash
set -euxo pipefail

nix build .#unremarkableNotes
ssh remarkable "killall unremarkable-notes || true"
scp ./result/bin/unremarkable-notes remarkable:
ssh remarkable "./unremarkable-notes --host 0.0.0.0"
