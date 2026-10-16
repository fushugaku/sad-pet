#!/bin/bash
set -euo pipefail
plugin_source="$(cd "$(dirname "$0")" && pwd)"
for plugin_type in AU VST3; do
    if [[ "$plugin_type" == AU ]]; then plugin_folder=Components; else plugin_folder=VST3; fi
    mkdir -p "$HOME/Library/Audio/Plug-Ins/$plugin_folder"
    for plugin_bundle in "$plugin_source/$plugin_type/"*; do
        [[ -d "$plugin_bundle" ]] || continue
        ditto "$plugin_bundle" "$HOME/Library/Audio/Plug-Ins/$plugin_folder/$(basename "$plugin_bundle")"
    done
done
# Refresh discovery after installing a new Audio Unit.
killall -9 AudioComponentRegistrar 2>/dev/null || true
echo "SAD PET and SAD PET FX installed. Restart your DAW and rescan plug-ins."
