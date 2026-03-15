# nu_plugin_audio justfile

# list available recipes
default:
    @just --list

# check formatting and clippy (run 'just fix' to auto-fix)
check:
    -cargo fmt --check
    -cargo clippy

# fix clippy warnings and format
fix:
    cargo clippy --fix --allow-dirty
    cargo fmt

# build (default features)
build:
    cargo build

# build with lite decoders
build-lite:
    cargo build --no-default-features --features=lite

# build release binary (default features)
build-release:
    cargo build --release --locked

# build release binary with lite decoders
build-release-lite:
    cargo build --release --locked --no-default-features --features=lite

# install optimized build via cargo install (*nix)
install-nx:
    cargo install --path . --locked
    nu -c "plugin add ~/.cargo/bin/nu_plugin_audio"

# install optimized build via cargo install (Windows)
install-win:
    cargo install --path . --locked
    nu -c "plugin add ~\\.cargo\\bin\\nu_plugin_audio.exe"

# install optimized build with lite decoders (*nix)
install-lite-nx:
    cargo install --path . --locked --no-default-features --features=lite
    nu -c "plugin add ~/.cargo/bin/nu_plugin_audio"

# install optimized build with lite decoders (Windows)
install-lite-win:
    cargo install --path . --locked --no-default-features --features=lite
    nu -c "plugin add ~\\.cargo\\bin\\nu_plugin_audio.exe"

# dry-run release (preview changelog, no publish)
release-dry:
    cargo smart-release --update-crates-index --changelog-without commit-statistics

# publish release to crates.io and push tag to github
release:
    cargo smart-release --update-crates-index --execute --changelog-without commit-statistics --no-tag
    git tag v$(cargo pkgid | cut -d# -f2)
    git push origin v$(cargo pkgid | cut -d# -f2)
