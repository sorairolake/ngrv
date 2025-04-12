// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    time::Duration,
};

use anyhow::Context;
use clap::Parser;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

use crate::cli::Opt;

/// Runs the program and returns the result.
#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let size = opt.size.map(u64::from);
    let interval = opt
        .interval
        .map(Duration::try_from)
        .transpose()
        .context("the interval is outside the range of valid values")?;
    let buffer_size = opt
        .buffer_size
        .map(usize::try_from)
        .transpose()
        .context("the buffer size is outside the range of valid values")?;
    let inputs = opt
        .inputs
        .filter(|paths| paths.iter().all(|path| path.as_os_str() != "-"));

    // The order is based on
    // <https://codeberg.org/ivarch/pv/src/tag/v1.9.31/src/pv/state.c#L274-L282>.
    let mut keys = Vec::with_capacity(7);
    if opt.name.is_some() {
        keys.push("{prefix:.bold}:");
    }
    if !opt.no_spinner {
        keys.push("{spinner:.green}");
    }
    if !opt.no_bytes {
        if opt.si {
            keys.push("{decimal_bytes}");
        } else {
            keys.push("{binary_bytes}");
        }
    }
    if !opt.no_timer {
        keys.push("{elapsed_precise}");
    }
    if !opt.no_rate {
        if opt.si {
            keys.push("[{decimal_bytes_per_sec}]");
        } else {
            keys.push("[{binary_bytes_per_sec}]");
        }
    }

    let stdout = io::stdout().lock();
    let writer = if let Some(buffer_size) = buffer_size {
        BufWriter::with_capacity(buffer_size, stdout)
    } else {
        BufWriter::new(stdout)
    };

    if let Some(paths) = inputs {
        let mut total_len = u64::default();
        let mut files = Vec::with_capacity(paths.len());

        for path in paths {
            let file =
                File::open(&path).with_context(|| format!("could not open {}", path.display()))?;

            let metadata = file
                .metadata()
                .context("could not query metadata about a file")?;
            total_len += metadata.len();

            files.push(file);
        }

        let total_size = size.unwrap_or(total_len);

        if !opt.no_progress {
            keys.push("{wide_bar:.cyan/blue} {percent}%");
        }
        if !opt.no_eta {
            keys.push("ETA {eta}");
        }
        let template = opt.format.unwrap_or_else(|| keys.join(" "));

        let mut style = ProgressStyle::with_template(&template)
            .context("could not set the template string for the progress bar")?;
        if let Some(string) = opt.bar_style {
            style = style.progress_chars(&string);
        }
        if let Some(strings) = opt.spinner_style {
            let strings: Vec<_> = strings.iter().map(String::as_str).collect();
            style = if let [string] = strings.as_slice() {
                style.tick_chars(string)
            } else {
                style.tick_strings(&strings)
            };
        }
        let pb = if opt.quiet {
            ProgressBar::hidden()
        } else {
            let pb = ProgressBar::new(total_size)
                .with_style(style)
                .with_finish(ProgressFinish::AndLeave);
            if let Some(interval) = interval {
                pb.enable_steady_tick(interval);
            }
            if let Some(name) = opt.name {
                pb.set_prefix(name);
            }
            pb
        };
        let mut writer = pb.wrap_write(writer);

        for file in files {
            let mut reader = if let Some(buffer_size) = buffer_size {
                BufReader::with_capacity(buffer_size, file)
            } else {
                BufReader::new(file)
            };

            io::copy(&mut reader, &mut writer)
                .context("could not copy the contents of a file to standard output")?;
        }
    } else {
        if !opt.no_progress && size.is_some() {
            keys.push("{wide_bar:.cyan/blue} {percent}%");
        }
        if !opt.no_eta && size.is_some() {
            keys.push("ETA {eta}");
        }
        let template = opt.format.unwrap_or_else(|| keys.join(" "));

        let mut style = ProgressStyle::with_template(&template)
            .context("could not set the template string for the progress bar")?;
        if let Some(string) = opt.bar_style {
            style = style.progress_chars(&string);
        }
        if let Some(strings) = opt.spinner_style {
            let strings: Vec<_> = strings.iter().map(String::as_str).collect();
            style = if let [string] = strings.as_slice() {
                style.tick_chars(string)
            } else {
                style.tick_strings(&strings)
            };
        }
        let pb = if opt.quiet {
            ProgressBar::hidden()
        } else {
            let pb = size
                .map_or_else(ProgressBar::no_length, ProgressBar::new)
                .with_style(style)
                .with_finish(ProgressFinish::AndLeave);
            if let Some(interval) = interval {
                pb.enable_steady_tick(interval);
            }
            if let Some(name) = opt.name {
                pb.set_prefix(name);
            }
            pb
        };
        let mut writer = pb.wrap_write(writer);

        let stdin = io::stdin().lock();
        let mut reader = if let Some(buffer_size) = buffer_size {
            BufReader::with_capacity(buffer_size, stdin)
        } else {
            BufReader::new(stdin)
        };

        io::copy(&mut reader, &mut writer)
            .context("could not copy the contents of standard input to standard output")?;
    }
    Ok(())
}
