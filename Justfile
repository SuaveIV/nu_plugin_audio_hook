# nu_plugin_audio justfile

# list available recipes
default:
    @just --list

# check formatting and clippy (run 'just fix' to auto-fix)
check:
    cargo fmt --check
    cargo clippy

# fix clippy warnings and format
fix:
    cargo clippy --fix --allow-dirty
    cargo fmt

# build (default features)
build:
    cargo build

# build with all decoders
build-all:
    cargo build --features=all-decoders

# build with lite decoders
build-lite:
    cargo build --no-default-features --features=lite

# build release binary (default features)
build-release:
    cargo build --release --locked

# build release binary with all decoders
build-release-all:
    cargo build --release --locked --features=all-decoders

# build release binary with lite decoders
build-release-lite:
    cargo build --release --locked --no-default-features --features=lite

# install debug build for development (default features)
dev:
    cargo build
    plugin add target/debug/nu_plugin_audio

# install debug build with all decoders
dev-all:
    cargo build --features=all-decoders
    plugin add target/debug/nu_plugin_audio

# install debug build with lite decoders
dev-lite:
    cargo build --no-default-features --features=lite
    plugin add target/debug/nu_plugin_audio

# install optimized build via cargo install (default features)
install:
    cargo install --path . --locked
    plugin add ~/.cargo/bin/nu_plugin_audio

# install optimized build with all decoders
install-all:
    cargo install --path . --locked --features=all-decoders
    plugin add ~/.cargo/bin/nu_plugin_audio

# install optimized build with lite decoders
install-lite:
    cargo install --path . --locked --no-default-features --features=lite
    plugin add ~/.cargo/bin/nu_plugin_audio

# dry-run release (preview changelog, no publish)
release-dry:
    cargo smart-release --update-crates-index --changelog-without commit-statistics

# publish release to crates.io and push tag to github
release:
    cargo smart-release --update-crates-index --execute --changelog-without commit-statistics
