set windows-shell := ["powershell.exe", "-NoProfile", "-NoLogo","-Command"]

default:
    @just --list

build:
    cargo build --workspace --release