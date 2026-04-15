# nu_plugin_audio justfile
set shell := ["nu", "-c"]

# list available recipes
default:
    @just --list

# verify if just is using nushell
test-shell:
    @echo $"Shell is (if ($nu != null) { 'Nushell' } else { 'Unknown' })"
    @echo $"Version: (version | get version)"

# check formatting and clippy (run 'just fix' to auto-fix)
check:
    -^cargo fmt --check
    -^cargo clippy

# fix clippy warnings and format
fix:
    ^cargo clippy --fix --allow-dirty
    ^cargo fmt

# build (default features)
build:
    ^cargo build

# build with lite decoders
build-lite:
    ^cargo build --no-default-features --features=lite

# build release binary (default features)
build-release:
    ^cargo build --release --locked

# build release binary with lite decoders
build-release-lite:
    ^cargo build --release --locked --no-default-features --features=lite

# install optimized build via cargo install (*nix)
install-nx:
    ^cargo install --path . --locked
    ^nu -c "plugin add ~/.cargo/bin/nu_plugin_audio"

# install optimized build via cargo install (Windows)
install-win:
    ^cargo install --path . --locked
    ^nu -c "plugin add ~\\.cargo\\bin\\nu_plugin_audio.exe"

# install optimized build with lite decoders (*nix)
install-lite-nx:
    ^cargo install --path . --locked --no-default-features --features=lite
    ^nu -c "plugin add ~/.cargo/bin/nu_plugin_audio"

# install optimized build with lite decoders (Windows)
install-lite-win:
    ^cargo install --path . --locked --no-default-features --features=lite
    ^nu -c "plugin add ~\\.cargo\\bin\\nu_plugin_audio.exe"

# dry-run release (preview changelog, no publish)
release-dry:
    ^cargo smart-release --update-crates-index --changelog-without commit-statistics

# publish release to crates.io and push tag to github
release:
    @echo "Ensuring cargo-dist is in sync..."
    ^dist init --yes
    @if (^git status --porcelain .github/workflows/release.yml dist-workspace.toml | is-not-empty) { \
        echo "Error: cargo-dist files were out of sync and have been updated."; \
        echo "Please commit these changes before running 'just release' again."; \
        exit 1; \
    }; ^cargo smart-release --update-crates-index --execute --changelog-without commit-statistics --no-tag
    @let version = (^cargo pkgid | str replace -r '.*#' '' | str replace -r '.*:' ''); \
    ^git tag $"v($version)" -m $"Release v($version)"; \
    ^git push origin main; \
    ^git push origin $"v($version)"
