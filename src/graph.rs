// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use anyhow::Context;
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressStyle};

const ROWS: usize = 4;
const DOTS_PER_ROW: usize = 4;
const TOTAL_LEVELS: usize = ROWS * DOTS_PER_ROW; // 16
const LABEL_COL: usize = 13; // " " + 12-char right-aligned rate label

// Braille dot bits ordered bottom-to-top for each column.
//
// Braille cell layout (Unicode U+2800 block):
//   dot1(0x01)  dot4(0x08)   ← top
//   dot2(0x02)  dot5(0x10)
//   dot3(0x04)  dot6(0x20)
//   dot7(0x40)  dot8(0x80)   ← bottom
const LEFT_BITS: [u8; DOTS_PER_ROW] = [0x40, 0x04, 0x02, 0x01];
const RIGHT_BITS: [u8; DOTS_PER_ROW] = [0x80, 0x20, 0x10, 0x08];

// IFStatus-inspired color scheme:
//   yellow = overlap region (both series reach this level) — the "confirmed" base
//   green  = per-second exclusive (instantaneous spike above the average)
//   cyan   = per-minute exclusive (average still high after a burst — rare)
const COLOR_BOTH: &str = "\x1b[93m"; // bright yellow: overlap
const COLOR_SEC: &str = "\x1b[92m";  // bright green:  per-second only
const COLOR_MIN: &str = "\x1b[96m";  // bright cyan:   per-minute only
const COLOR_RESET: &str = "\x1b[0m";

#[derive(Clone, Copy, PartialEq, Eq)]
enum BarColor {
    Both,
    Sec,
    Min,
}

impl BarColor {
    const fn ansi(self) -> &'static str {
        match self {
            Self::Both => COLOR_BOTH,
            Self::Sec => COLOR_SEC,
            Self::Min => COLOR_MIN,
        }
    }
}

/// Configuration for graph mode, assembled from CLI options in `app::run`.
pub struct GraphArgs {
    /// Partial template keys (name, spinner, bytes, timer, rate) — progress bar
    /// and ETA keys are appended here depending on whether the total size is known.
    pub keys: Vec<&'static str>,
    pub size: Option<u64>,
    pub interval: Option<Duration>,
    pub buffer_size: Option<usize>,
    /// `None` means read from stdin; `Some` holds the resolved input file paths.
    pub inputs: Option<Vec<PathBuf>>,
    pub no_progress: bool,
    pub no_eta: bool,
    pub bar_style: Option<String>,
    pub spinner_style: Option<Vec<String>>,
    pub format: Option<String>,
    pub name: Option<String>,
    pub si: bool,
}

/// Runs graph mode end-to-end: sets up the multi-progress display, spawns the
/// throughput-sampling thread, performs the I/O copy, and cleans up.
pub fn run(args: GraphArgs) -> anyhow::Result<()> {
    let GraphArgs {
        mut keys,
        size,
        interval,
        buffer_size,
        inputs,
        no_progress,
        no_eta,
        bar_style,
        spinner_style,
        format,
        name,
        si,
    } = args;

    let multi = MultiProgress::new();
    let done = Arc::new(AtomicBool::new(false));

    if let Some(paths) = inputs {
        let mut total_len = u64::default();
        let mut files = Vec::with_capacity(paths.len());
        for path in &paths {
            let file = File::open(path)
                .with_context(|| format!("could not open {}", path.display()))?;
            total_len += file
                .metadata()
                .context("could not query metadata about a file")?
                .len();
            files.push(file);
        }
        let total_size = size.unwrap_or(total_len);

        if !no_progress {
            keys.push("{wide_bar:.cyan/blue} {percent}%");
        }
        if !no_eta {
            keys.push("ETA {eta}");
        }
        let template = format.unwrap_or_else(|| keys.join(" "));
        let mut style = ProgressStyle::with_template(&template)
            .context("could not set the template string for the progress bar")?;
        if let Some(string) = bar_style {
            style = style.progress_chars(&string);
        }
        if let Some(strings) = spinner_style {
            let strings: Vec<_> = strings.iter().map(String::as_str).collect();
            style = if let [string] = strings.as_slice() {
                style.tick_chars(string)
            } else {
                style.tick_strings(&strings)
            };
        }
        let pb = multi.add(
            ProgressBar::new(total_size)
                .with_style(style)
                .with_finish(ProgressFinish::AndLeave),
        );
        if let Some(iv) = interval {
            pb.enable_steady_tick(iv);
        }
        if let Some(n) = name {
            pb.set_prefix(n);
        }

        let graph_rows = add_graph_rows(&multi);
        let handle = spawn_graph_updater(pb.clone(), graph_rows, Arc::clone(&done), si);

        let stdout = io::stdout().lock();
        let out = if let Some(bs) = buffer_size {
            BufWriter::with_capacity(bs, stdout)
        } else {
            BufWriter::new(stdout)
        };
        let mut writer = pb.wrap_write(out);

        for file in files {
            let mut reader = if let Some(bs) = buffer_size {
                BufReader::with_capacity(bs, file)
            } else {
                BufReader::new(file)
            };
            io::copy(&mut reader, &mut writer)
                .context("could not copy the contents of a file to standard output")?;
        }

        done.store(true, Ordering::Relaxed);
        let _ = handle.join();
    } else {
        if !no_progress && size.is_some() {
            keys.push("{wide_bar:.cyan/blue} {percent}%");
        }
        if !no_eta && size.is_some() {
            keys.push("ETA {eta}");
        }
        let template = format.unwrap_or_else(|| keys.join(" "));
        let mut style = ProgressStyle::with_template(&template)
            .context("could not set the template string for the progress bar")?;
        if let Some(string) = bar_style {
            style = style.progress_chars(&string);
        }
        if let Some(strings) = spinner_style {
            let strings: Vec<_> = strings.iter().map(String::as_str).collect();
            style = if let [string] = strings.as_slice() {
                style.tick_chars(string)
            } else {
                style.tick_strings(&strings)
            };
        }
        let pb = multi.add(
            size.map_or_else(ProgressBar::no_length, ProgressBar::new)
                .with_style(style)
                .with_finish(ProgressFinish::AndLeave),
        );
        if let Some(iv) = interval {
            pb.enable_steady_tick(iv);
        }
        if let Some(n) = name {
            pb.set_prefix(n);
        }

        let graph_rows = add_graph_rows(&multi);
        let handle = spawn_graph_updater(pb.clone(), graph_rows, Arc::clone(&done), si);

        let stdout = io::stdout().lock();
        let out = if let Some(bs) = buffer_size {
            BufWriter::with_capacity(bs, stdout)
        } else {
            BufWriter::new(stdout)
        };
        let mut writer = pb.wrap_write(out);

        let stdin = io::stdin().lock();
        let mut reader = if let Some(bs) = buffer_size {
            BufReader::with_capacity(bs, stdin)
        } else {
            BufReader::new(stdin)
        };
        io::copy(&mut reader, &mut writer)
            .context("could not copy the contents of standard input to standard output")?;

        done.store(true, Ordering::Relaxed);
        let _ = handle.join();
    }

    Ok(())
}

fn add_graph_rows(multi: &MultiProgress) -> Vec<ProgressBar> {
    (0..ROWS)
        .map(|_| {
            let pb = ProgressBar::new(0);
            pb.set_style(
                ProgressStyle::with_template("{wide_msg}")
                    .expect("graph row template is always valid"),
            );
            multi.add(pb)
        })
        .collect()
}

fn spawn_graph_updater(
    main_pb: ProgressBar,
    graph_rows: Vec<ProgressBar>,
    done: Arc<AtomicBool>,
    si: bool,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut sec_history: VecDeque<u64> = VecDeque::new();
        let mut min_history: VecDeque<u64> = VecDeque::new();
        let mut last_pos = 0u64;
        let mut last_sample = Instant::now();

        loop {
            thread::sleep(Duration::from_millis(100));
            let is_done = done.load(Ordering::Relaxed);

            if last_sample.elapsed() >= Duration::from_secs(1) || is_done {
                let current = main_pb.position();
                sec_history.push_back(current.saturating_sub(last_pos));
                last_pos = current;
                last_sample = Instant::now();

                // 60-second rolling average, time-aligned with sec_history.
                let n = sec_history.len().min(60);
                let sum: u64 = sec_history.iter().rev().take(n).sum();
                min_history.push_back(sum / n as u64);

                let lines = render_graph(&sec_history, &min_history, si);
                for (row_pb, line) in graph_rows.iter().zip(lines.iter()) {
                    row_pb.set_message(line.clone());
                }
            }

            if is_done {
                for row_pb in &graph_rows {
                    row_pb.finish();
                }
                break;
            }
        }
    })
}

/// Formats bytes per second with the appropriate SI or binary prefix.
#[expect(clippy::cast_precision_loss)]
fn format_rate(bps: u64, si: bool) -> String {
    if si {
        if bps >= 1_000_000_000 {
            format!("{:.1} GB/s", bps as f64 / 1_000_000_000.0)
        } else if bps >= 1_000_000 {
            format!("{:.1} MB/s", bps as f64 / 1_000_000.0)
        } else if bps >= 1_000 {
            format!("{:.1} kB/s", bps as f64 / 1_000.0)
        } else {
            format!("{bps} B/s")
        }
    } else if bps >= 1_073_741_824 {
        format!("{:.1} GiB/s", bps as f64 / 1_073_741_824.0)
    } else if bps >= 1_048_576 {
        format!("{:.1} MiB/s", bps as f64 / 1_048_576.0)
    } else if bps >= 1_024 {
        format!("{:.1} KiB/s", bps as f64 / 1_024.0)
    } else {
        format!("{bps} B/s")
    }
}

/// Renders the 4-line dual-series braille graph with a right-side Y-axis.
///
/// Color scheme (IFStatus-inspired):
///   yellow = both series reach this level (overlap / confirmed base)
///   green  = per-second spike above the 60-second average
///   cyan   = per-minute average above the current instantaneous (rare)
fn render_graph(sec_history: &VecDeque<u64>, min_history: &VecDeque<u64>, si: bool) -> [String; ROWS] {
    let width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);

    // Scale is driven by the per-second peak so bars can reach 100 %.
    let max = sec_history.iter().copied().max().unwrap_or(1).max(1);

    let graph_width = width.saturating_sub(LABEL_COL);
    let start = sec_history.len().saturating_sub(graph_width * 2);
    let mut sec_vis: Vec<u64> = sec_history.range(start..).copied().collect();
    let mut min_vis: Vec<u64> = min_history.range(start..).copied().collect();

    // Align so the newest sample always lands on the right column of the last character.
    // Without this, an odd-length window puts the newest sample on the left column and
    // leaves the right column empty, creating a "tick" artifact on every scroll step.
    if sec_vis.len() % 2 == 1 {
        sec_vis.insert(0, 0);
        min_vis.insert(0, 0);
    }

    // Each braille char encodes 2 consecutive time slots (left + right dot column).
    let cols: Vec<[(char, Option<BarColor>); ROWS]> = sec_vis
        .chunks(2)
        .enumerate()
        .map(|(i, s)| {
            let s_l = s[0];
            let s_r = s.get(1).copied().unwrap_or(0);
            let m_l = min_vis.get(i * 2).copied().unwrap_or(0);
            let m_r = min_vis.get(i * 2 + 1).copied().unwrap_or(0);
            render_col_dual(s_l, s_r, m_l, m_r, max)
        })
        .collect();

    let pad = graph_width.saturating_sub(cols.len());
    let peak_label = format!(" {:>12}", format_rate(max, si));
    let mid_label = format!(" {:>12}", format_rate(max / 2, si));
    let empty_label = " ".repeat(LABEL_COL);

    std::array::from_fn(|r| {
        let mut row = " ".repeat(pad);
        let mut cur: Option<BarColor> = None;

        for col in &cols {
            let (ch, next) = col[r];
            match (cur, next) {
                // Leaving a colored run → reset.
                (Some(_), None) => {
                    row.push_str(COLOR_RESET);
                    cur = None;
                    row.push(ch);
                }
                // Continuing no-color (space).
                (None, None) => {
                    row.push(ch);
                }
                // Same color as before → no escape needed.
                (Some(prev), Some(nxt)) if prev == nxt => {
                    row.push(ch);
                }
                // Color change (new or different).
                (_, Some(nxt)) => {
                    row.push_str(nxt.ansi());
                    row.push(ch);
                    cur = Some(nxt);
                }
            }
        }

        if cur.is_some() {
            row.push_str(COLOR_RESET);
        }

        let label = match r {
            0 => &peak_label,
            2 => &mid_label,
            _ => &empty_label,
        };
        row.push_str(label);
        row
    })
}

/// Maps two time-slot pairs (per-second and 60 s average) onto one column of
/// braille characters with per-row color annotations.
///
/// The braille glyph shows the envelope `max(sec, min)`.
/// Color: yellow where both series contribute, green for sec-only rows,
/// cyan for min-only rows.
fn render_col_dual(
    s_l: u64,
    s_r: u64,
    m_l: u64,
    m_r: u64,
    max: u64,
) -> [(char, Option<BarColor>); ROWS] {
    let sf_l = (s_l * TOTAL_LEVELS as u64 / max) as usize;
    let sf_r = (s_r * TOTAL_LEVELS as u64 / max) as usize;
    let mf_l = (m_l * TOTAL_LEVELS as u64 / max) as usize;
    let mf_r = (m_r * TOTAL_LEVELS as u64 / max) as usize;

    std::array::from_fn(|r| {
        let base = (ROWS - 1 - r) * DOTS_PER_ROW;

        let sd_l = sf_l.saturating_sub(base).min(DOTS_PER_ROW);
        let sd_r = sf_r.saturating_sub(base).min(DOTS_PER_ROW);
        let md_l = mf_l.saturating_sub(base).min(DOTS_PER_ROW);
        let md_r = mf_r.saturating_sub(base).min(DOTS_PER_ROW);

        let ch = make_braille(sd_l.max(md_l), sd_r.max(md_r));

        let s_here = sd_l > 0 || sd_r > 0;
        let m_here = md_l > 0 || md_r > 0;
        let color = match (s_here, m_here) {
            (true, true) => Some(BarColor::Both),
            (true, false) => Some(BarColor::Sec),
            (false, true) => Some(BarColor::Min),
            (false, false) => None,
        };

        (ch, color)
    })
}

/// Assembles a braille Unicode character from filled dot counts (0–4 each).
/// Dots are filled bottom-to-top independently for each column.
fn make_braille(dots_left: usize, dots_right: usize) -> char {
    let mut bits = 0u8;
    for i in 0..dots_left {
        bits |= LEFT_BITS[i];
    }
    for i in 0..dots_right {
        bits |= RIGHT_BITS[i];
    }
    if bits == 0 {
        return ' ';
    }
    char::from_u32(0x2800 | u32::from(bits)).unwrap_or(' ')
}
