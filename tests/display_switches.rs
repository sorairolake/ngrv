// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn no_progress() {
    command::command()
        .arg("--no-progress")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_progress_conflicts_with() {
    command::command()
        .arg("--no-progress")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-progress' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-progress")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-progress' cannot be used with '--quiet'",
        ));
    command::command()
        .arg("--no-progress")
        .arg("-u")
        .arg("#>-")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-progress' cannot be used with '--bar-style <STRING>'",
        ));
}

#[test]
fn no_timer() {
    command::command()
        .arg("--no-timer")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_timer_conflicts_with() {
    command::command()
        .arg("--no-timer")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-timer' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-timer")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-timer' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_eta() {
    command::command()
        .arg("--no-eta")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_eta_conflicts_with() {
    command::command()
        .arg("--no-eta")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-eta' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-eta")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-eta' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_rate() {
    command::command()
        .arg("--no-rate")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_rate_conflicts_with() {
    command::command()
        .arg("--no-rate")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-rate' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-rate")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-rate' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_bytes() {
    command::command()
        .arg("--no-bytes")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_bytes_conflicts_with() {
    command::command()
        .arg("--no-bytes")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-bytes' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-bytes")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-bytes' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_spinner() {
    command::command()
        .arg("--no-spinner")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn no_spinner_conflicts_with() {
    command::command()
        .arg("--no-spinner")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--format <FORMAT>'",
        ));
    command::command()
        .arg("--no-spinner")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--quiet'",
        ));
    command::command()
        .arg("--no-spinner")
        .arg("--spinner-style")
        .arg(r"/|\- ")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--spinner-style <STRING>...'",
        ));
    command::command()
        .arg("--no-spinner")
        .arg("--spinner-style")
        .arg("▹▹▹▹▹")
        .arg("▸▹▹▹▹")
        .arg("▹▸▹▹▹")
        .arg("▹▹▸▹▹")
        .arg("▹▹▹▸▹")
        .arg("▹▹▹▹▸")
        .arg("▪▪▪▪▪")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--spinner-style <STRING>...'",
        ));
}

#[test]
fn format() {
    command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn format_conflicts_with() {
    command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--format <FORMAT>' cannot be used with '--quiet'",
        ));
    command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("-k")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--format <FORMAT>' cannot be used with '--si'",
        ));
}

#[test]
fn quiet() {
    command::command()
        .arg("-q")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn quiet_conflicts_with() {
    command::command()
        .arg("-q")
        .arg("-k")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--si'",
        ));
    command::command()
        .arg("-q")
        .arg("-s")
        .arg("128MiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--size <SIZE>'",
        ));
    command::command()
        .arg("-q")
        .arg("-i")
        .arg("1s")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--interval <SEC>'",
        ));
    command::command()
        .arg("-q")
        .arg("-N")
        .arg("foo")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--name <NAME>'",
        ));
    command::command()
        .arg("-q")
        .arg("-u")
        .arg("#>-")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--bar-style <STRING>'",
        ));
    command::command()
        .arg("-q")
        .arg("--spinner-style")
        .arg(r"/|\- ")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--spinner-style <STRING>...'",
        ));
    command::command()
        .arg("-q")
        .arg("--spinner-style")
        .arg("▹▹▹▹▹")
        .arg("▸▹▹▹▹")
        .arg("▹▸▹▹▹")
        .arg("▹▹▸▹▹")
        .arg("▹▹▹▸▹")
        .arg("▹▹▹▹▸")
        .arg("▪▪▪▪▪")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--spinner-style <STRING>...'",
        ));
}
