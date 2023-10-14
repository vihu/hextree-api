# Define the default settings.toml file path

default-settings := "settings.toml"

default:
    just --list --unsorted

# Build the entire project
build:
    cargo build --release

# Runs server
server settings=default-settings:
    ./target/release/hextree-api -c {{ settings }} serve

# Combines yarn and server commands
serve settings=default-settings:
    @just build
    @just server {{ settings }}
