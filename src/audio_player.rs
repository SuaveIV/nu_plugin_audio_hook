use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Attribute, SetAttribute},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::Accessor;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};
use rodio::{source::Source, Decoder, DeviceSinkBuilder, Player};

use std::io::{stderr, Write};
use std::time::{Duration, Instant};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::{
    utils::{format_duration, load_file},
    Sound,
};

/// Interval for checking keyboard input.
const KEY_POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Interval for updating the progress display (to reduce flicker).
const RENDER_INTERVAL: Duration = Duration::from_millis(500);

/// Amount to seek forward or backward when FF/RWD is pressed.
const SEEK_STEP: Duration = Duration::from_secs(5);

/// Minimum duration for interactive controls to be shown.
const CONTROLS_THRESHOLD: Duration = Duration::from_secs(60);

/// How much to change volume per keypress (5%).
const VOLUME_STEP: f32 = 0.05;

/// Maximum volume (200%).
const VOLUME_MAX: f32 = 2.0;

const WIDTH_FULL: u16 = 80;
const WIDTH_COMPACT: u16 = 50;
const WIDTH_MINIMAL: u16 = 30;
const WIDTH_BARE: u16 = 20;
const MARQUEE_GAP: &str = "     ";

/// Selects the glyph set used for the live progress display.
///
/// Priority order for resolution: `--nerd-fonts` flag → `NERD_FONTS=1` env var →
/// Unicode (if the terminal locale advertises UTF-8) → ASCII fallback.
#[derive(Clone, Copy, PartialEq)]
enum IconSet {
    /// Nerd Font glyphs — richest, requires a patched font.
    NerdFont,
    /// Standard Unicode block/arrow characters — works on most modern terminals.
    Unicode,
    /// Pure ASCII — works everywhere.
    Ascii,
}

impl IconSet {
    /// Play icon: `▶` / `>`.
    fn play(&self) -> &'static str {
        match self {
            Self::NerdFont => "\u{f04b}",
            Self::Unicode => "▶",
            Self::Ascii => ">",
        }
    }
    /// Pause icon: `⏸` / `||`.
    fn pause(&self) -> &'static str {
        match self {
            Self::NerdFont => "\u{f04c}",
            Self::Unicode => "⏸",
            Self::Ascii => "||",
        }
    }
    /// Rewind / seek-back icon: `«` / `<<`.
    fn rewind(&self) -> &'static str {
        match self {
            Self::NerdFont => "\u{f04a}",
            Self::Unicode => "«",
            Self::Ascii => "<<",
        }
    }
    /// Fast-forward / seek-forward icon: `»` / `>>`.
    fn fast_forward(&self) -> &'static str {
        match self {
            Self::NerdFont => "\u{f04e}",
            Self::Unicode => "»",
            Self::Ascii => ">>",
        }
    }
    /// Music note / track decoration icon.
    fn music(&self) -> &'static str {
        match self {
            Self::NerdFont => "\u{f001}",
            Self::Unicode => "♪",
            Self::Ascii => "#",
        }
    }
    /// Filled bar segment.
    fn fill(&self) -> &'static str {
        match self {
            Self::NerdFont => "█",
            Self::Unicode => "█",
            Self::Ascii => "#",
        }
    }
    /// Empty bar segment.
    fn empty(&self) -> &'static str {
        match self {
            Self::NerdFont => "░",
            Self::Unicode => "░",
            Self::Ascii => ".",
        }
    }

    /// Volume icon — three tiers based on level.
    fn volume(&self, level: f32) -> &'static str {
        match self {
            Self::NerdFont => {
                if level == 0.0 {
                    "\u{f026}"
                }
                // nf-fa-volume_off
                else if level < 0.5 {
                    "\u{f027}"
                }
                // nf-fa-volume_down
                else {
                    "\u{f028}"
                } // nf-fa-volume_up
            }
            Self::Unicode => {
                if level == 0.0 {
                    "🔇"
                } else if level < 0.5 {
                    "🔉"
                } else {
                    "🔊"
                }
            }
            Self::Ascii => {
                if level == 0.0 {
                    "[M]"
                }
                // muted
                else if level < 0.5 {
                    "[v]"
                } else {
                    "[V]"
                }
            }
        }
    }
}

/// Nushell command `sound play` — decodes and plays an audio file with a live
/// progress bar on stderr and optional interactive keyboard controls.
pub struct SoundPlayCmd;
impl SimplePluginCommand for SoundPlayCmd {
    type Plugin = Sound;

    fn name(&self) -> &str {
        "sound play"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("sound play")
            .required("File Path", SyntaxShape::Filepath, "file to play")
            .named(
                "duration",
                SyntaxShape::Duration,
                "truncate playback to this duration (default: auto-detected from file headers)",
                Some('d'),
            )
            .named(
                "amplify",
                SyntaxShape::Float,
                "initial volume: 1.0 = normal, 0.5 = half, 2.0 = double (default 1.0)",
                Some('a'),
            )
            .switch(
                "no-progress",
                "disable live playback stats (use when piping or running in background)",
                Some('q'),
            )
            .switch(
                "nerd-fonts",
                "use Nerd Font icons in the progress display (or set NERD_FONTS=1)",
                Some('n'),
            )
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "play a sound and exit after 5min",
                example: "sound play audio.mp4 -d 5min",
                result: None,
            },
            Example {
                description: "play a sound starting at 2x volume",
                example: "sound play audio.mp3 -a 2.0",
                result: None,
            },
            Example {
                description: "play a sound starting at 50% volume",
                example: "sound play audio.mp3 -a 0.5",
                result: None,
            },
            Example {
                description: "play a sound for its detected metadata duration",
                example: "sound play audio.mp3",
                result: None,
            },
            Example {
                description: "play silently — no terminal output (background or pipe use)",
                example: "sound play audio.mp3 --no-progress",
                result: None,
            },
            Example {
                description: "play with Nerd Font icons",
                example: "sound play audio.mp3 --nerd-fonts",
                result: None,
            },
        ]
    }

    fn description(&self) -> &str {
        "play an audio file; by default supports FLAC, WAV, MP3, OGG, AAC, and MP4 files \
        (install with `all-decoders` feature to include minimp3, 64-bit precision, and more). \
        Displays live playback stats by default; use --no-progress (-q) to suppress \
        output for scripting or background use. Interactive controls (space, arrows) \
        are available for files longer than 1 minute, including volume up/down and 5s seeking. \
        Use --nerd-fonts (-n) or set NERD_FONTS=1 for richer icons."
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        play_audio(engine, call).map(|_| Value::nothing(call.head))
    }
}

// ---------------------------------------------------------------------------
// Core playback
// ---------------------------------------------------------------------------

/// Opens the default audio output, decodes the file via rodio, and delegates to
/// either [`wait_silent`] or [`wait_with_progress`] depending on `--no-progress`.
///
/// Duration is resolved in priority order: `-d` flag → `source.total_duration()` →
/// `lofty::FileProperties::duration()` → 1-hour safety fallback.
fn play_audio(engine: &EngineInterface, call: &EvaluatedCall) -> Result<(), LabeledError> {
    let (file_span, file, path) = load_file(engine, call)?;

    let mut output_stream = DeviceSinkBuilder::open_default_sink().map_err(|err| {
        LabeledError::new(err.to_string()).with_label("audio stream exception", call.head)
    })?;

    output_stream.log_on_drop(false);

    let source = Decoder::try_from(file).map_err(|err| {
        LabeledError::new(err.to_string()).with_label("audio decoder exception", file_span)
    })?;

    // Read the tagged file once; reuse the result for both metadata and duration fallback.
    let tagged_file_res = lofty::read_from_path(&path);
    let (title, artist) = tagged_file_res
        .as_ref()
        .ok()
        .and_then(|tf| tf.primary_tag())
        .map(|tag| {
            (
                tag.title().map(|s| s.to_string()),
                tag.artist().map(|s| s.to_string()),
            )
        })
        .unwrap_or((None, None));

    // Volume is now set on the Player rather than baked into the source with
    // amplify(), so it can be changed live and survives seeks correctly.
    let initial_volume: f32 = match call.get_flag_value("amplify") {
        Some(Value::Float { val, .. }) => (val as f32).clamp(0.0, VOLUME_MAX),
        _ => 1.0,
    };

    // Prefer rodio's own duration; fall back to lofty's container-header duration
    // so that minimp3 (which cannot seek-scan) still reports the correct length
    // without needing a manual -d flag.
    let source_duration: Option<Duration> = source.total_duration().or_else(|| {
        tagged_file_res
            .ok()
            .map(|tf| tf.properties().duration())
            .filter(|d| !d.is_zero())
    });

    let sink = Player::connect_new(output_stream.mixer());
    sink.append(source);
    sink.set_volume(initial_volume as _);

    let sleep_duration: Duration = match load_duration_from(call, "duration") {
        Some(d) => d,
        None => match source_duration {
            Some(d) => d,
            None => Duration::from_secs(3600),
        },
    };

    let no_progress = call.has_flag("no-progress").unwrap_or(false);

    if no_progress {
        wait_silent(engine, call, &sink, sleep_duration)
    } else {
        let icon_set = resolve_icon_set(call);
        let ctx = WaitProgressContext {
            engine,
            call,
            sink: &sink,
            total: sleep_duration,
            initial_volume,
            icons: icon_set,
            title,
            artist,
        };
        wait_with_progress(ctx)
    }
}

// ---------------------------------------------------------------------------
// Icon set resolution
// ---------------------------------------------------------------------------

/// Resolves the icon set to use, in priority order:
///   1. `--nerd-fonts` flag
///   2. `NERD_FONTS=1` environment variable
///   3. Unicode if the terminal locale supports UTF-8
///   4. ASCII fallback
fn resolve_icon_set(call: &EvaluatedCall) -> IconSet {
    let flag = call.has_flag("nerd-fonts").unwrap_or(false);
    let env = std::env::var("NERD_FONTS")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    if flag || env {
        return IconSet::NerdFont;
    }

    if terminal_supports_unicode() {
        IconSet::Unicode
    } else {
        IconSet::Ascii
    }
}

// ---------------------------------------------------------------------------
// Wait strategies
// ---------------------------------------------------------------------------

/// Waits for playback to finish without rendering any output to stderr.
///
/// Exits early when `sink.empty()` returns `true` so the command returns promptly
/// at the real end of the stream rather than sleeping for the full `total` duration.
fn wait_silent(
    engine: &EngineInterface,
    call: &EvaluatedCall,
    sink: &Player,
    total: Duration,
) -> Result<(), LabeledError> {
    let start = Instant::now();

    while start.elapsed() < total && !sink.empty() {
        engine.signals().check(&call.head)?;
        std::thread::sleep(KEY_POLL_INTERVAL);
    }

    Ok(())
}

/// Context for the wait_with_progress function, grouping all its parameters.
///
/// Holds the state required to drive the playback loop and render the UI.
struct WaitProgressContext<'a> {
    engine: &'a EngineInterface,
    call: &'a EvaluatedCall,
    sink: &'a Player,
    total: Duration,
    initial_volume: f32,
    icons: IconSet,
    title: Option<String>,
    artist: Option<String>,
}

/// Renders a live progress line (and optional header) to stderr while the sink plays.
///
/// For files longer than [`CONTROLS_THRESHOLD`] the terminal is placed in raw mode and
/// keyboard events (space, arrows, `m`, `q`) are processed. Raw mode is always restored
/// on exit, even if an error occurs.
///
/// Delegates the actual drawing to [`render_progress`], managing the `scroll_offset`
/// state for the header marquee animation and the `first_render` initialization flag.
fn wait_with_progress(ctx: WaitProgressContext) -> Result<(), LabeledError> {
    let mut err = stderr();
    let interactive = ctx.total >= CONTROLS_THRESHOLD;

    let mut position = Duration::ZERO;
    let mut last_render = Instant::now()
        .checked_sub(RENDER_INTERVAL)
        .unwrap_or(Instant::now());
    let mut paused = false;
    let mut volume = ctx.initial_volume;
    let mut pre_mute_volume = ctx.initial_volume;
    let mut header_reserved = false;
    let mut scroll_offset: usize = 0;

    let _ = execute!(err, Hide);

    // Pre-compute the header string once; render_progress will redraw it every frame.
    let header: Option<String> = {
        let parts: Vec<&str> = [ctx.artist.as_deref(), ctx.title.as_deref()]
            .into_iter()
            .flatten()
            .collect();

        if !parts.is_empty() {
            let header_text = parts.join(" — ");
            let prefix = format!("{}  ", ctx.icons.music());
            Some(format!("{}{}", prefix, header_text))
        } else {
            None
        }
    };

    if interactive {
        if let Err(e) = enable_raw_mode() {
            let _ = execute!(err, Show);
            return Err(LabeledError::new(e.to_string())
                .with_label("failed to enable raw terminal mode", ctx.call.head));
        }
    }

    let result = (|| {
        loop {
            // Cap at total: some codecs briefly report a position slightly
            // beyond the stream duration, which would incorrectly trip the
            // end-of-track check or clamp the progress bar to 100% too early.
            position = ctx.sink.get_pos().min(ctx.total);

            if position >= ctx.total || ctx.sink.empty() {
                break;
            }

            ctx.engine.signals().check(&ctx.call.head)?;

            let mut needs_render = false;

            if interactive && event::poll(Duration::ZERO).unwrap_or(false) {
                if let Ok(Event::Key(KeyEvent { code, kind, .. })) = event::read() {
                    if kind == event::KeyEventKind::Press {
                        match code {
                            // Space — toggle play/pause.
                            KeyCode::Char(' ') => {
                                if paused {
                                    ctx.sink.play();
                                    paused = false;
                                } else {
                                    ctx.sink.pause();
                                    paused = true;
                                }
                                needs_render = true;
                            }
                            // Right / 'l' — seek forward.
                            KeyCode::Right | KeyCode::Char('l') => {
                                let target = (position + SEEK_STEP).min(ctx.total);
                                let _ = ctx.sink.try_seek(target);
                                needs_render = true;
                            }
                            // Left / 'h' — seek backward.
                            KeyCode::Left | KeyCode::Char('h') => {
                                let target = position.saturating_sub(SEEK_STEP);
                                let _ = ctx.sink.try_seek(target);
                                needs_render = true;
                            }
                            // Up / 'k' — volume up.
                            KeyCode::Up | KeyCode::Char('k') => {
                                volume = (volume + VOLUME_STEP).min(VOLUME_MAX);
                                if volume > 0.0 {
                                    pre_mute_volume = volume;
                                }
                                ctx.sink.set_volume(volume as _);
                                needs_render = true;
                            }
                            // Down / 'j' — volume down.
                            KeyCode::Down | KeyCode::Char('j') => {
                                volume = (volume - VOLUME_STEP).max(0.0);
                                if volume > 0.0 {
                                    pre_mute_volume = volume;
                                }
                                ctx.sink.set_volume(volume as _);
                                needs_render = true;
                            }
                            // 'm' — toggle mute (sets volume to 0 / restores).
                            KeyCode::Char('m') => {
                                if volume > 0.0 {
                                    pre_mute_volume = volume;
                                    volume = 0.0;
                                } else {
                                    volume = pre_mute_volume.max(VOLUME_STEP);
                                }
                                ctx.sink.set_volume(volume as _);
                                needs_render = true;
                            }
                            // 'q' / Escape — stop.
                            KeyCode::Char('q') | KeyCode::Esc => {
                                ctx.sink.stop();
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            if needs_render || last_render.elapsed() >= RENDER_INTERVAL {
                let render_ctx = RenderProgressContext {
                    err: &mut err,
                    elapsed: position,
                    total: ctx.total,
                    paused,
                    volume,
                    interactive,
                    icons: &ctx.icons,
                    header: header.as_deref(),
                    header_reserved,
                    scroll_offset,
                };
                let rendered = render_progress(render_ctx);
                if rendered {
                    if let Some(hdr) = &header {
                        let term_width = size().map(|(w, _)| w).unwrap_or(u16::MAX);
                        if term_width >= WIDTH_COMPACT {
                            header_reserved = true;
                        }
                        if term_width >= WIDTH_COMPACT && marquee_needed(hdr, term_width) {
                            let cycle_len = hdr.chars().count() + MARQUEE_GAP.chars().count();
                            scroll_offset = (scroll_offset + 1) % cycle_len;
                        }
                    }
                }
                last_render = Instant::now();
            }
            std::thread::sleep(KEY_POLL_INTERVAL);
        }

        let final_render_ctx = RenderProgressContext {
            err: &mut err,
            elapsed: position.min(ctx.total),
            total: ctx.total,
            paused: false,
            volume,
            interactive,
            icons: &ctx.icons,
            header: header.as_deref(),
            header_reserved,
            scroll_offset,
        };
        render_progress(final_render_ctx);
        Ok::<(), LabeledError>(())
    })();

    if interactive {
        let _ = disable_raw_mode();
    }
    if header_reserved {
        let _ = execute!(err, MoveToColumn(0), Clear(ClearType::CurrentLine));
        let _ = execute!(
            err,
            Show,
            MoveUp(1),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine)
        );
    } else {
        let _ = execute!(err, Show, MoveToColumn(0), Clear(ClearType::CurrentLine));
    }

    result
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Context for the render_progress function, grouping all its parameters.
///
/// Includes `scroll_offset` for marquee rendering of the header.
struct RenderProgressContext<'a> {
    err: &'a mut std::io::Stderr,
    elapsed: Duration,
    total: Duration,
    paused: bool,
    volume: f32,
    interactive: bool,
    icons: &'a IconSet,
    header: Option<&'a str>,
    header_reserved: bool,
    scroll_offset: usize,
}

fn marquee_needed(header: &str, term_width: u16) -> bool {
    header.width() > term_width as usize
}

/// Renders one progress line in-place on stderr.
///
/// Returns `true` if rendering occurred, or `false` if the terminal was too narrow
/// (less than [`WIDTH_BARE`]).
///
/// # Layout Tiers
///
/// The layout adapts to the terminal width:
/// - **Full** (≥80 cols): Progress bar, volume bar, and control hints.
/// - **Compact** (≥50 cols): Progress bar and volume bar, no hints.
/// - **Minimal** (≥30 cols): Text status only (`0:00/1:00 50%`).
/// - **Bare** (≥20 cols): Percent only (`50%`).
///
/// # Examples
///
/// ```text
/// Full:    ♪ ▶  0:42 / 4:05  [████████░░░░░░░░░░░░░░░░░░░░░░]  17%  🔊 [████░░] 100%  « [SPACE] »  [q]
/// Compact: ♪ ▶  0:42 / 4:05  [████████░░░░░░░░░░░░░░░░░░░░░░]  17%  🔊 [████░░] 100%
/// Minimal:  0:42/4:05 17%
/// Bare:     17%
/// ```
fn render_progress(ctx: RenderProgressContext) -> bool {
    // Bail out silently on very narrow terminals rather than wrapping garbage.
    let term_size = size();
    let term_width = term_size.as_ref().map(|(w, _)| *w).unwrap_or(u16::MAX);
    if term_width < WIDTH_BARE {
        return false;
    }

    let elapsed_str = format_duration(ctx.elapsed);
    let total_str = format_duration(ctx.total);
    let ratio = if ctx.total.is_zero() {
        0.0
    } else {
        (ctx.elapsed.as_secs_f64() / ctx.total.as_secs_f64()).clamp(0.0, 1.0)
    };
    let percent = (ratio * 100.0).round() as u8;

    let prefix = if *ctx.icons == IconSet::NerdFont {
        format!("{} ", ctx.icons.music())
    } else {
        String::new()
    };
    let icon = if ctx.paused {
        ctx.icons.pause()
    } else {
        ctx.icons.play()
    };

    // Build the entire output (header + progress line) into a single buffer so
    // it is written to the terminal in one write_all + flush — eliminating the
    // partial-state flicker that multiple separate write!/queue! calls cause on
    // Windows.
    let mut buf: Vec<u8> = Vec::new();

    if let Some(hdr) = ctx.header {
        if term_width >= WIDTH_COMPACT {
            if !ctx.header_reserved {
                // Reserve a blank line that will become the header line.  The
                // cursor ends up one line below it, which is exactly where the
                // progress line lives from this point on.
                let _ = buf.write_all(b"\n");
            }
            // Move up to the header line, clear it, and redraw.
            let _ = queue!(buf, MoveUp(1));
            let _ = queue!(buf, MoveToColumn(0));

            if marquee_needed(hdr, term_width) {
                let mut current_width = 0;
                let mut visible = String::new();
                for c in hdr
                    .chars()
                    .chain(MARQUEE_GAP.chars())
                    .cycle()
                    .skip(ctx.scroll_offset)
                {
                    let w = c.width().unwrap_or(0);
                    if current_width + w > term_width as usize {
                        break;
                    }
                    current_width += w;
                    visible.push(c);
                }
                let _ = buf.write_all(visible.as_bytes());
            } else {
                let _ = buf.write_all(hdr.as_bytes());
            }

            let _ = queue!(buf, Clear(ClearType::UntilNewLine));
            // Drop back down to the progress line.
            let _ = buf.write_all(b"\n");
        } else if ctx.header_reserved {
            // Terminal has narrowed below compact tier but the header line was
            // already reserved — move up, clear it, and drop back down so it
            // does not sit frozen above the progress line.
            let _ = queue!(buf, MoveUp(1));
            let _ = queue!(buf, MoveToColumn(0));
            let _ = queue!(buf, Clear(ClearType::UntilNewLine));
            let _ = buf.write_all(b"\n");
        }
    }

    // Redraw the progress line.
    let _ = queue!(buf, MoveToColumn(0));
    let _ = queue!(buf, SetAttribute(Attribute::Bold));
    let _ = buf.write_all(format!("{prefix}{icon}").as_bytes());
    let _ = queue!(buf, SetAttribute(Attribute::Reset));

    if term_width >= WIDTH_COMPACT {
        let vol_pct = (ctx.volume.min(VOLUME_MAX) * 100.0).round() as u8;
        let vol_icon = ctx.icons.volume(ctx.volume);

        let controls_suffix = if term_width >= WIDTH_FULL && ctx.interactive {
            let toggle_label = if ctx.paused { "play " } else { "pause" };
            format!(
                "  {} [SPACE/{toggle_label}] {}  [↑↓/kj] vol  [m] mute  [q] quit",
                ctx.icons.rewind(),
                ctx.icons.fast_forward(),
            )
        } else {
            String::new()
        };

        // Dynamic width calculation
        let (bar_width, vol_bar_width) = if let Ok((cols, _)) = term_size {
            let overhead = prefix.width()
                + icon.width()
                + 2 // "  "
                + elapsed_str.width()
                + 3 // " / "
                + total_str.width()
                + 2 // "  "
                + 2 // "[]" main bar
                + 2 // "  "
                + percent.to_string().width()
                + 1 // "%"
                + 2 // "  "
                + vol_icon.width()
                + 1 // " "
                + 2 // "[]" vol bar
                + 1 // " "
                + vol_pct.to_string().width()
                + 1 // "%"
                + controls_suffix.width();

            let available = (cols as usize).saturating_sub(overhead);
            // We want: bar_width + vol_bar_width <= available
            // vol_bar_width = max(5, bar_width / 3)
            // If bar_width >= 15, vol = bar_width / 3. Total = 4/3 * bar_width.
            // If bar_width < 15, vol = 5. Total = bar_width + 5.

            let min_vol = 5usize;
            let target = (available * 3) / 4;
            let tentative = if target >= 15 {
                target
            } else {
                available.saturating_sub(min_vol)
            };
            let bar_width = tentative.clamp(0, 60);
            let vol_bar_width = (bar_width / 3).max(min_vol);
            // Final guard: ensure bar_width + vol_bar_width never exceeds available.
            let bar_width = if bar_width + vol_bar_width > available {
                available.saturating_sub(vol_bar_width)
            } else {
                bar_width
            };
            (bar_width, vol_bar_width)
        } else {
            (30, 10)
        };

        let bar = render_bar(ratio, bar_width, ctx.icons);
        let vol_ratio = (ctx.volume as f64 / VOLUME_MAX as f64).clamp(0.0, 1.0);
        let vol_bar = render_bar(vol_ratio, vol_bar_width, ctx.icons);

        let _ = buf.write_all(
            format!("  {elapsed_str} / {total_str}  {bar}  {percent}%  {vol_icon} {vol_bar} {vol_pct}%{controls_suffix}")
                .as_bytes(),
        );
    } else if term_width >= WIDTH_MINIMAL {
        let _ = buf.write_all(format!(" {elapsed_str}/{total_str} {percent}%").as_bytes());
    } else {
        // WIDTH_BARE
        let _ = buf.write_all(format!(" {percent}%").as_bytes());
    }

    let _ = queue!(buf, Clear(ClearType::UntilNewLine));

    let _ = ctx.err.write_all(&buf);
    let _ = ctx.err.flush();
    true
}

/// Renders a single progress bar of the given `width` as a `String`.
///
/// For [`IconSet::NerdFont`] a fractional leading block character is used for
/// sub-cell precision; other icon sets round to the nearest whole cell.
fn render_bar(ratio: f64, width: usize, icons: &IconSet) -> String {
    let ratio = ratio.clamp(0.0, 1.0);
    let f_width = ratio * width as f64;

    let n_full = if *icons == IconSet::NerdFont {
        (f_width.floor() as usize).min(width)
    } else {
        (f_width.round() as usize).min(width)
    };

    let bytes_per_char = match icons {
        IconSet::Ascii => 1,
        _ => 3, // NerdFont and Unicode use 3-byte UTF-8 chars (e.g. █ U+2588, ░ U+2591)
    };
    let mut s = String::with_capacity(width * bytes_per_char + 2);
    s.push('[');

    for _ in 0..n_full {
        s.push_str(icons.fill());
    }

    let mut current_len = n_full;

    if current_len < width && *icons == IconSet::NerdFont {
        let remainder = f_width - n_full as f64;
        let part_idx = (remainder * 8.0).floor() as usize;
        if part_idx > 0 {
            let partials = ['▏', '▎', '▍', '▌', '▋', '▊', '▉'];
            if part_idx <= partials.len() {
                s.push(partials[part_idx - 1]);
                current_len += 1;
            }
        }
    }

    while current_len < width {
        s.push_str(icons.empty());
        current_len += 1;
    }

    s.push(']');
    s
}

/// Returns `true` if the current terminal environment is likely to support Unicode.
fn terminal_supports_unicode() -> bool {
    #[cfg(target_os = "windows")]
    {
        std::env::var("WT_SESSION").is_ok()
            || std::env::var("ConEmuPID").is_ok()
            || std::env::var("TERM_PROGRAM")
                .map(|v| v == "vscode")
                .unwrap_or(false)
            || std::env::var("ANSICON").is_ok()
    }

    #[cfg(not(target_os = "windows"))]
    {
        let lang = std::env::var("LANG")
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default()
            .to_uppercase();
        lang.contains("UTF-8") || lang.contains("UTF8")
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Reads a nushell `Duration` flag by name and converts it to [`std::time::Duration`].
/// Returns `None` if the flag was not supplied or contains a negative value.
fn load_duration_from(call: &EvaluatedCall, name: &str) -> Option<Duration> {
    match call.get_flag_value(name) {
        Some(Value::Duration { val, .. }) if val >= 0 => Some(Duration::from_nanos(val as u64)),
        _ => None,
    }
}
