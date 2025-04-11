// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{self, Write},
    path::PathBuf,
};

use byte_unit::Byte;
use clap::{CommandFactory, Parser, ValueEnum, ValueHint};
use clap_complete::Generator;
use jiff::Span;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    '\n',
    "Copyright (C) 2025 Shun Sakai\n",
    '\n',
    "This program is distributed under the terms of the GNU General Public License\n",
    "v3.0 or later.\n",
    '\n',
    "This is free software: you are free to change and redistribute it. There is NO\n",
    "WARRANTY, to the extent permitted by law.\n",
    '\n',
    "Report bugs to <https://github.com/sorairolake/ngrv/issues>."
);

const AFTER_LONG_HELP: &str = "See `ngrv(1)` for more details.";

#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    after_long_help(AFTER_LONG_HELP)
)]
pub struct Opt {
    /// Hide the progress bar.
    ///
    /// If any inputs are not files, or are unreadable, and no size was
    /// explicitly given with '--size', the progress bar cannot indicate how
    /// close to completion the transfer is, so it will not be shown even if
    /// this option is not specified.
    #[arg(
        long,
        conflicts_with("format"),
        conflicts_with("quiet"),
        conflicts_with("bar_style")
    )]
    pub no_progress: bool,

    /// Hide the elapsed time.
    #[arg(long, conflicts_with("format"), conflicts_with("quiet"))]
    pub no_timer: bool,

    /// Hide the remaining time.
    ///
    /// This will estimate, based on current transfer rates and the total data
    /// size, how long it will be before completion. The remaining time is
    /// prefixed with "ETA". If the total data size cannot be determined, the
    /// remaining time will not be shown even if this option is not specified.
    #[arg(long, conflicts_with("format"), conflicts_with("quiet"))]
    pub no_eta: bool,

    /// Hide the speed of data transfer.
    ///
    /// If this option is not specified, The rate will be shown in square
    /// brackets "[]".
    #[arg(long, conflicts_with("format"), conflicts_with("quiet"))]
    pub no_rate: bool,

    /// Hide the total amount of data transferred.
    #[arg(long, conflicts_with("format"), conflicts_with("quiet"))]
    pub no_bytes: bool,

    /// Hide the spinner.
    #[arg(
        long,
        conflicts_with("format"),
        conflicts_with("quiet"),
        conflicts_with("spinner_style")
    )]
    pub no_spinner: bool,

    /// Set output format to <FORMAT>.
    #[arg(
        short('F'),
        long,
        value_name("FORMAT"),
        conflicts_with("quiet"),
        conflicts_with("si")
    )]
    pub format: Option<String>,

    /// Don't output any transfer information at all.
    #[arg(
        short,
        long,
        conflicts_with("si"),
        conflicts_with("size"),
        conflicts_with("interval"),
        conflicts_with("name"),
        conflicts_with("bar_style"),
        conflicts_with("spinner_style")
    )]
    pub quiet: bool,

    /// Use binary prefixes rather than SI prefixes for the amount of data
    /// displayed in the progress.
    #[arg(short('k'), long)]
    pub si: bool,

    /// Set estimated data size to <SIZE> bytes.
    ///
    /// <SIZE> can be suffixed with a symbol (B), which represents the byte. B
    /// can be prefixed with SI prefixes or binary prefixes.
    #[arg(short, long, value_name("SIZE"))]
    pub size: Option<Byte>,

    /// Wait <SEC> seconds between updates.
    #[arg(short, long, value_name("SEC"))]
    pub interval: Option<Span>,

    /// Prefix the output information with <NAME>.
    #[arg(short('N'), long, value_name("NAME"))]
    pub name: Option<String>,

    /// Set the progress characters to <STRING>.
    #[arg(short('u'), long, value_name("STRING"))]
    pub bar_style: Option<String>,

    /// Set the tick character sequence or the tick string sequence for the
    /// spinner to <STRING>.
    ///
    /// If this option takes multiple <STRING>, it sets the tick string
    /// sequence, otherwise it sets the tick character sequence.
    #[arg(long, num_args(1..), value_name("STRING"))]
    pub spinner_style: Option<Vec<String>>,

    /// Use a transfer buffer size of <BYTES> bytes.
    ///
    /// The same suffixes as '--size' can be used.
    #[arg(short('B'), long, value_name("BYTES"))]
    pub buffer_size: Option<Byte>,

    /// Generate shell completion.
    ///
    /// The completion is output to standard output.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    /// Files to concatenate.
    ///
    /// If [FILE] is not specified, or if "-" is specified, data will be read
    /// from standard input.
    #[arg(value_name("FILE"), value_hint(ValueHint::FilePath))]
    pub inputs: Option<Vec<PathBuf>>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(generator: impl Generator) {
        clap_complete::generate(
            generator,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    #[allow(clippy::enum_variant_names)]
    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("ngrv"), "ngrv.bash");
        assert_eq!(Shell::Elvish.file_name("ngrv"), "ngrv.elv");
        assert_eq!(Shell::Fish.file_name("ngrv"), "ngrv.fish");
        assert_eq!(Shell::Nushell.file_name("ngrv"), "ngrv.nu");
        assert_eq!(Shell::PowerShell.file_name("ngrv"), "_ngrv.ps1");
        assert_eq!(Shell::Zsh.file_name("ngrv"), "_ngrv");
    }
}
