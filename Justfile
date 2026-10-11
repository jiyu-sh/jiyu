list:
    @just --list

check:
    cargo check --workspace

build:
    cargo build --release --workspace

run *arguments:
    cargo run --release --package jiyu-cli -- {{ arguments }}

nix-check:
    nix flake check

nix-build:
    nix build

nix-run *arguments:
    nix run . -- {{ arguments }}
