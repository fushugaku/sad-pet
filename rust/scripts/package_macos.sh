#!/usr/bin/env bash
set -euo pipefail
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$project_dir"
cargo build --release --locked
app_dir="$project_dir/dist/SAD PET.app"
mkdir -p "$app_dir/Contents/MacOS"
cp "$project_dir/target/release/sad-pet" "$app_dir/Contents/MacOS/sad-pet.new"
mv -f "$app_dir/Contents/MacOS/sad-pet.new" "$app_dir/Contents/MacOS/sad-pet"
cp "$project_dir/rust/macos/Info.plist" "$app_dir/Contents/Info.plist"
mkdir -p "$app_dir/Contents/Resources"
cp -R "$project_dir/rust/licenses" "$app_dir/Contents/Resources/"
codesign --force --sign - "$app_dir"
ditto -c -k --sequesterRsrc --keepParent "$app_dir" "$project_dir/dist/SAD-PET-macOS.zip"
printf '%s\n' "$app_dir"
