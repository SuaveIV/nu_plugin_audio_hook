# Contributing to nu_plugin_audio

Thanks for your interest in contributing! This document covers everything you need to get started.

---

## Getting started

### Prerequisites

- A stable Rust toolchain (`rustup` is recommended — see [rust-toolchain.toml](rust-toolchain.toml))
- On Linux, the ALSA development package:

  | Distro | Command |
  | --- | --- |
  | Debian / Ubuntu | `sudo apt install libasound2-dev pkg-config` |
  | Fedora / RHEL | `sudo dnf install alsa-lib-devel pkgconf-pkg-config` |
  | Arch | `sudo pacman -S alsa-lib pkgconf` |
  | openSUSE | `sudo zypper install alsa-lib-devel pkg-config` |

### Building

```bash
git clone https://github.com/SuaveIV/nu_plugin_audio.git
cd nu_plugin_audio
cargo build
```

To build with every decoder enabled:

```bash
cargo build --features=all-decoders
```

### Running locally

```nushell
cargo build
plugin add target/debug/nu_plugin_audio
```

> You'll need to restart Nushell (or run `plugin use nu_plugin_audio`) after registering for the first time.

---

## Commit messages

This project uses **[Conventional Commits](https://www.conventionalcommits.org/)**.
Commit messages are parsed automatically to generate the changelog on each release, so please follow the format.

### Format

```markdown
<type>(<optional scope>): <short description>

<optional body>

<optional footer>
```

### Types

| Type | When to use |
| --- | --- |
| `feat` | A new feature or command |
| `fix` | A bug fix |
| `docs` | Documentation changes only |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `perf` | Performance improvements |
| `test` | Adding or fixing tests |
| `chore` | Maintenance — dependency bumps, CI, build scripts |
| `style` | Formatting, whitespace — no logic changes |

### Examples

```sh
feat(meta): add bit_depth field to sound meta output
fix(player): clamp position to total duration to prevent overflow
docs: add CONTRIBUTING.md
chore: bump nu-plugin to 0.111.0
refactor(utils): extract format_duration into shared utils module
```

### Breaking changes

If your change breaks the public command interface, add `!` after the type and include a `BREAKING CHANGE:` footer:

```sh
feat(play)!: rename --amplify to --volume

BREAKING CHANGE: the -a / --amplify flag has been renamed to -v / --volume
to better match Nushell conventions.
```

Breaking changes will be called out explicitly in the generated changelog.

---

## Pull requests

- **One thing per PR** — focused changes are much easier to review and bisect if something goes wrong.
- **Target `main`** — all PRs should be opened against the main branch.
- **Update the README** if you add or change command flags, behaviour, or supported formats.
- There are no formal tests right now. If you add one, great — but it's not a blocker.

---

## Releasing (maintainers only)

Releases are managed with [cargo-smart-release](https://github.com/Byron/gitoxide/tree/main/cargo-smart-release).

```bash
cargo smart-release --update-crates-index --execute
```

This will:

1. Parse conventional commits since the last tag and write `CHANGELOG.md`
2. Bump the version in `Cargo.toml`
3. Publish to [crates.io](https://crates.io/crates/nu_plugin_audio)
4. Push the commit and version tag to GitHub

The tag push then triggers the release CI, which builds platform binaries via cargo-dist and creates the GitHub release with the changelog as its body.

> Make sure your working tree is clean and you're on `main` before running this.

---

## Code style

- Run `cargo fmt` before committing.
- Run `cargo clippy` and address any warnings.
- Public functions and types should have doc comments (`///`).

---

## Just

This project uses a [`Justfile`](Justfile) for common tasks. If you have [just](https://github.com/casey/just) installed:
```just --list``` to see all available recipes.

The most useful ones for contributors are `just check` before committing and `just install` to test a local build in Nushell.

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list of people who have contributed to this project.
