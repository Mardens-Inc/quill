set windows-shell := ["powershell.exe", "-NoProfile", "-NoLogo", "-Command"]

default:
    @just --list

[working-directory("./crates/quill_configuration_app")]
install:
    @pnpm i

build: && _build_tauri _dist
    @cargo build --workspace --release

_build_tauri: install
    @cargo tauri build

dev: install
    @cargo tauri dev

clean: _clean_node
    @echo "Cleaning cargo project..."
    @cargo clean

[windows]
[working-directory("./target/")]
_dist:
    @if (Test-Path ./dist){Remove-Item ./dist -Recurse -Force}
    @mkdir dist
    @cp ./release/quill_configuration_app.exe, ./release/quill_server.exe, ./release/quill_server.exe, ./release/bundle/nsis/*, ./release/bundle/msi/* ./dist

[windows]
[working-directory("./crates/quill_configuration_app")]
_clean_node:
    @Write-Host "Cleaning node modules..."
    @if (Test-Path ./node_modules){Remove-Item ./node_modules -Recurse -Force}

[linux]
[working-directory("./crates/quill_configuration_app")]
_clean_node:
    @echo "Cleaning node modules..."
    @rm -rf ./node_modules

[macos]
[working-directory("./crates/quill_configuration_app")]
_clean_node:
    @echo "Cleaning node modules..."
    @rm -rdf ./node_modules
