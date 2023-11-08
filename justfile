# Muestra la lista de commandos disponibles
default:
    @just --list

run *args:
    cargo run {{args}}

# cargo test -- --nocapture
test *args:
    cargo test {{args}}

pre-commit:
    cargo fmt --all
    cargo clippy --fix --allow-dirty --tests -- -D clippy::all

    pre-commit install
    pre-commit run --all-files
