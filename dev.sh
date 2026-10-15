#!/usr/bin/env bash
set -euo pipefail

evil_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$evil_root/rust/scripts/dev.py" "$@"
