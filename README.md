# nu_plugin_audio

[![Nushell](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FSuaveIV%2Fnu_plugin_audio%2Fmain%2FCargo.toml&query=%24.dependencies%5B%22nu-plugin%22%5D.version&label=nushell&color=blue&logo=nushell)](https://www.nushell.sh/)
[![crates.io](https://img.shields.io/crates/v/nu_plugin_audio.svg)](https://crates.io/crates/nu_plugin_audio)
[![Downloads](https://img.shields.io/crates/d/nu_plugin_audio.svg)](https://crates.io/crates/nu_plugin_audio)

A [Nushell](https://www.nushell.sh/) plugin to generate and play sounds. It supports tone generation, metadata manipulation, and playback for multiple audio formats.

## Origins and Attribution

This project is a continuation and expansion of the original `nu_plugin_audio_hook` created by [fmotalleb](https://github.com/fmotalleb/nu_plugin_audio_hook). Due to real-world circumstances, the original author plans to archive their repository and step away from development. They gave explicit permission to fork the project and keep the code alive. I am incredibly grateful for their hard work in laying the foundation for this tool.

---

## Features

- `sound beep` - Play a simple beep sound.
- `sound make` - Generate a noise with a given frequency and duration.
- `sound meta` - Retrieve metadata (duration, artist, album, etc.) from an audio file.
- `sound meta set` - Modify metadata tags in an audio file using format-agnostic key names.
- `sound play` - Play an audio file with a live progress display and interactive controls. By default, it supports FLAC, WAV, MP3, OGG, AAC, and MP4 playback. Use the `all-decoders` feature for an expanded set like 64-bit support.

---

## Usage

### Generate a simple noise

```bash
sound make 1000 200ms
```

### Generate a noise sequence

```bash
[ 300.0, 500.0, 1000.0, 400.0, 600.0 ] | each { |it| sound make $it 150ms }
```

### Generate a noise with 50% volume

```bash
sound make 1000 200ms -a 0.5
```

### Save a generated tone to a file

```bash
sound make 1000 200ms --data | save --raw output.wav
```

### Play an audio file (first 3 seconds only)

```bash
sound play audio.mp3 -d 3sec
```

### Play an audio file starting at 2x volume

```bash
sound play audio.mp3 -a 2.0
```

### Play an audio file starting at 50% volume

```bash
sound play audio.mp3 -a 0.5
```

### Play silently (no terminal output for scripts or background tasks)

```bash
sound play audio.mp3 --no-progress
```

### Play with Nerd Font icons

```bash
sound play audio.mp3 --nerd-fonts
```

### Retrieve metadata from an audio file

```bash
sound meta audio.mp3
```

Example output:

```nushell
╭───────────────┬────────────────────────────╮
│ size          │ 6.4 MiB                    │
│ format        │ mp3                        │
│ bitrate       │ 320                        │
│ audio_bitrate │ 320                        │
│ artist        │ SINGER                     │
│ title         │ TITLE                      │
│ album         │ ALBUM                      │
│ albumartist   │ SINGER                     │
│ comment       │ Tagged with MusicBrainz    │
│ date          │ 2024-03-15                 │
│ genre         │ Rock                       │
│ track_no      │ 1                          │
│ total_tracks  │ 12                         │
│ artwork       │ [list 1 item]              │
│ duration      │ 4:05                       │
│ sample_rate   │ 44100                      │
│ channels      │ 2                          │
╰───────────────┴────────────────────────────╯
```

The `artwork` field is a list of records, one per embedded image:

```nushell
sound meta audio.mp3 | get artwork
# ╭───┬───────────────┬────────────┬──────────╮
# │ # │ pic_type      │ mime_type  │ size     │
# ├───┼───────────────┼────────────┼──────────┤
# │ 0 │ CoverFront    │ image/jpeg │ 127.3 KB │
# ╰───┴───────────────┴────────────┴──────────╯
```

FLAC and lossless files additionally expose `bit_depth`:

```nushell
sound meta audio.flac | select size format bitrate bit_depth
# ╭───────────┬──────────╮
# │ size      │ 42.3 MiB │
# │ format    │ flac     │
# │ bitrate   │ 1411     │
# │ bit_depth │ 24       │
# ╰───────────┴──────────╯
```

### Modify metadata (change the artist tag)

```bash
sound meta set audio.mp3 -k artist -v "new-artist"
```

Key names are case-insensitive. `artist`, `Artist`, and `ARTIST` all work. Key names are format-agnostic. The same key works across MP3, FLAC, OGG, and MP4 files. Use `sound meta --all` to list every available key name.

### Set a comment tag

```bash
sound meta set audio.mp3 -k comment -v "ripped from vinyl"
```

### Set ReplayGain values

```bash
sound meta set audio.mp3 -k replaygain_track_gain -v "-6.2 dB"
sound meta set audio.mp3 -k replaygain_track_peak -v "0.998"
```

### List all available metadata key names

```bash
sound meta --all
```

Key names are normalised to lowercase before lookup, so `Artist`, `ARTIST`, and `artist` are all accepted. The table below shows every supported key grouped by category.

#### Core identity

| Key | Maps to |
| --- | --- |
| `album` | Album title |
| `albumartist` | Album-level artist |
| `albumsortorder` | Album title sort order |
| `artist` | Track artist |
| `artistsortorder` | Track artist sort order |
| `title` | Track title |
| `titlesortorder` | Track title sort order |
| `subtitle` | Track subtitle |
| `setsubtitle` | Set/disc subtitle |

#### People & roles

| Key | Maps to |
| --- | --- |
| `composer` | Composer |
| `composersortorder` | Composer sort order |
| `conductor` | Conductor |
| `label` | Record label |
| `lyricist` | Lyricist |
| `movement` | Movement name |
| `movementnumber` | Movement number |
| `movementtotal` | Total movements |
| `organization` | Publisher |
| `producer` | Producer |
| `publisher` | Publisher (alias for `organization`) |
| `remixer` | Remixer / mix artist |
| `work` | Work title |

#### Dates

| Key | Maps to |
| --- | --- |
| `date` | Recording date (ISO 8601, e.g. `2024-03-15`) |
| `originalyear` | Original release date |
| `releasedate` | Release date |
| `year` | Release year (bare integer) |

#### Identifiers

| Key | Maps to |
| --- | --- |
| `barcode` | Release barcode (EAN/UPC) |
| `cataloguenumber` | Catalogue number |
| `isrc` | ISRC |

#### Style & content

| Key | Maps to |
| --- | --- |
| `bpm` | BPM (decimal string) |
| `comment` | Comment |
| `compilation` | Compilation flag (`1` / `0`) |
| `copyright` | Copyright message |
| `discnumber` | Disc number |
| `encodedby` | Encoded by |
| `encodingsettings` | Encoder settings |
| `genre` | Genre |
| `grouping` | Content group / grouping |
| `initialkey` | Initial key (e.g. `Am`) |
| `language` | Language |
| `lyrics` | Lyrics text |
| `mood` | Mood |
| `originalalbum` | Original album title |
| `originalartist` | Original artist |
| `script` | Script (e.g. `Latin`) |
| `track` | Track number |

#### ReplayGain

| Key | Maps to |
| --- | --- |
| `replaygain_album_gain` | Album gain (dB string, e.g. `-6.5 dB`) |
| `replaygain_album_peak` | Album peak (float string, e.g. `0.998`) |
| `replaygain_track_gain` | Track gain |
| `replaygain_track_peak` | Track peak |

---

## Live Playback Display

When you play a file, `sound play` renders a live progress bar to stderr:

```nushell
▶  0:42 / 4:05  [██████████░░░░░░░░░░░░░░░░░░░░]  17%  🔊 [████████░░░░░░] 100%
```

Because the display writes to stderr, stdout remains clean. Piping the result of `sound play` to another command works without any garbled output. Use `--no-progress` (`-q`) to suppress the display entirely.

### Nerd Font mode

If you have a [Nerd Font](https://www.nerdfonts.com) installed and configured in your terminal, pass `--nerd-fonts` (`-n`) or set `NERD_FONTS=1` in your environment for richer icons:

```nushell
  0:42 / 4:05  [██████████░░░░░░░░░░░░░░░░░░░░]  17%   [████████░░░░░░] 100%
```

To enable permanently, add this to your `env.nu`:

```nushell
$env.NERD_FONTS = "1"
```

---

## Interactive Controls

For files longer than **1 minute**, interactive keyboard controls are enabled automatically:

| Key | Action |
| --- | --- |
| `Space` | Play / pause |
| `→` or `l` | Seek forward 5 seconds |
| `←` or `h` | Seek backward 5 seconds |
| `↑` or `k` | Volume up 5% |
| `↓` or `j` | Volume down 5% |
| `m` | Toggle mute |
| `q` or `Esc` | Stop and quit |

The control hint is shown inline on the progress bar and updates live to reflect the current state:

```nushell
▶  0:42 / 4:05  [██████████░░░░░░░░░░░░░░░░░░░░]  17%  🔊 [████████░░░░░░] 100%  « [SPACE/pause] »  [↑↓/kj] vol  [m] mute  [q] quit
```

Use `--no-progress` to disable terminal output and controls. This is recommended when you run tasks in the background or pipe output.

---

## Installation

### Linux requirements (ALSA)

Before installing on Linux, make sure the ALSA development package is present:

| Distro | Command |
| --- | --- |
| Debian / Ubuntu | `sudo apt install libasound2-dev pkg-config` |
| Fedora / RHEL | `sudo dnf install alsa-lib-devel pkgconf-pkg-config` |
| Arch | `sudo pacman -S alsa-lib pkgconf` |
| openSUSE | `sudo zypper install alsa-lib-devel pkg-config` |

### Using [nupm](https://github.com/nushell/nupm) (Recommended)

```nushell
git clone https://github.com/SuaveIV/nu_plugin_audio.git
nupm install --path nu_plugin_audio -f
```

### Shell Installer (Linux / macOS)

No Rust toolchain required. Run this in your terminal, then register the plugin in Nushell:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/SuaveIV/nu_plugin_audio/releases/latest/download/nu_plugin_audio-installer.sh | sh
```

```nushell
plugin add ~/.cargo/bin/nu_plugin_audio
```

> **Note:** The shell installer covers x86_64 Linux, x86_64 macOS, and ARM64 macOS (Apple Silicon).
> ARM64 Linux (`aarch64-unknown-linux-gnu`) is not included in the installer — use the
> [Manual Download](#manual-download) or [`cargo install`](#cargo-install) method instead.

### PowerShell Installer (Windows)

No Rust toolchain required. Run this in PowerShell, then register the plugin in Nushell:

```powershell
irm https://github.com/SuaveIV/nu_plugin_audio/releases/latest/download/nu_plugin_audio-installer.ps1 | iex
```

```nushell
plugin add ($env.USERPROFILE | path join ".cargo" "bin" "nu_plugin_audio.exe")
```

### cargo-binstall

Installs a prebuilt binary from the GitHub release, falling back to a source build automatically:

```nushell
cargo binstall nu_plugin_audio
plugin add ~/.cargo/bin/nu_plugin_audio
```

### cargo install

Builds from source using the published crate on [crates.io](https://crates.io/crates/nu_plugin_audio):

```nushell
cargo install nu_plugin_audio --locked
plugin add ~/.cargo/bin/nu_plugin_audio
```

### Manual Download

Download the prebuilt binary for your platform from the [Releases page](https://github.com/SuaveIV/nu_plugin_audio/releases), extract it, and register it:

```nushell
plugin add path/to/nu_plugin_audio
```

Prebuilt binaries are available for the following targets:

| Target | Notes |
| --- | --- |
| `x86_64-unknown-linux-gnu` | x86_64 Linux |
| `aarch64-unknown-linux-gnu` | ARM64 Linux — built with `--features=all-decoders` |
| `x86_64-apple-darwin` | x86_64 macOS |
| `aarch64-apple-darwin` | ARM64 macOS (Apple Silicon) |
| `x86_64-pc-windows-msvc` | x86_64 Windows |

Each archive has a matching `.sha256` checksum file on the release page.

### Compile from Source

Only needed if you want custom feature flags:

```nushell
git clone https://github.com/SuaveIV/nu_plugin_audio.git
cd nu_plugin_audio
cargo build -r --locked --features=all-decoders
plugin add target/release/nu_plugin_audio
```

---

## Supported formats

### Default install

Enabled out of the box with no extra flags (includes all Symphonia codecs):

| Format | Feature flag | Notes |
| --- | --- | --- |
| MP3 | `symphonia-all` | Via Symphonia |
| FLAC | `symphonia-all` | Lossless compression |
| OGG Vorbis | `symphonia-all` | Open lossy format |
| WAV | `symphonia-all` | Uncompressed PCM |
| AAC | `symphonia-all` | Used by Apple, YouTube, most streaming services |
| MP4 / M4A | `symphonia-all` | Container for AAC and ALAC |
| ALAC | `symphonia-all` | Apple Lossless |
| ADPCM | `symphonia-all` | Adaptive PCM; common in games |
| CAF | `symphonia-all` | Core Audio Format; Apple professional audio |
| MKV / WebM (Opus) | `symphonia-all` | Open container with Opus codec |

### With `--features=all-decoders`

Everything above plus expanded capabilities:

| Format | Feature flag | Notes |
| --- | --- | --- |
| 64-bit precision | `64bit` | f64 sample precision |

> **Note:** The `default` feature includes `rodio/symphonia-all`, providing support for almost every common audio format out of the box.

### Compile with specific formats only

Users who want a smaller binary can build with the `lite` feature, which only includes a minimal set of decoders:

```bash
cargo build -r --locked --no-default-features --features=lite
```

> **Note:** `cargo install` always uses the `default` feature set. Custom features require a source build.

---

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list of contributors.
