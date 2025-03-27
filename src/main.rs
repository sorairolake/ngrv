// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod cli;

use std::{io, process::ExitCode};

fn main() -> ExitCode {
    sigpipe::reset();

    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            if let Some(e) = err.downcast_ref::<io::Error>() {
                return sysexits::ExitCode::from(e.kind()).into();
            }
            ExitCode::FAILURE
        }
    }
}
