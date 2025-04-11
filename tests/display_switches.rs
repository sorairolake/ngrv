// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn no_progress() {
    utils::command::command()
        .arg("--no-progress")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_progress_conflicts_with() {
    utils::command::command()
        .arg("--no-progress")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-progress' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-progress")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-progress' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_timer() {
    utils::command::command()
        .arg("--no-timer")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_timer_conflicts_with() {
    utils::command::command()
        .arg("--no-timer")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-timer' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-timer")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-timer' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_eta() {
    utils::command::command()
        .arg("--no-eta")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_eta_conflicts_with() {
    utils::command::command()
        .arg("--no-eta")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-eta' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-eta")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-eta' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_rate() {
    utils::command::command()
        .arg("--no-rate")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_rate_conflicts_with() {
    utils::command::command()
        .arg("--no-rate")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-rate' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-rate")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-rate' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_bytes() {
    utils::command::command()
        .arg("--no-bytes")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_bytes_conflicts_with() {
    utils::command::command()
        .arg("--no-bytes")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-bytes' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-bytes")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-bytes' cannot be used with '--quiet'",
        ));
}

#[test]
fn no_spinner() {
    utils::command::command()
        .arg("--no-spinner")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn no_spinner_conflicts_with() {
    utils::command::command()
        .arg("--no-spinner")
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--format <FORMAT>'",
        ));
    utils::command::command()
        .arg("--no-spinner")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--no-spinner' cannot be used with '--quiet'",
        ));
}

#[test]
fn format() {
    utils::command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn format_conflicts_with() {
    utils::command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--format <FORMAT>' cannot be used with '--quiet'",
        ));
    utils::command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("-k")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--format <FORMAT>' cannot be used with '--si'",
        ));
    utils::command::command()
        .arg("-F")
        .arg("{wide_bar}")
        .arg("-N")
        .arg("foo")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--format <FORMAT>' cannot be used with '--name <NAME>'",
        ));
}

#[test]
fn quiet() {
    utils::command::command()
        .arg("-q")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn quiet_conflicts_with() {
    utils::command::command()
        .arg("-q")
        .arg("-k")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--si'",
        ));
    utils::command::command()
        .arg("-q")
        .arg("-s")
        .arg("128MiB")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--size <SIZE>'",
        ));
    utils::command::command()
        .arg("-q")
        .arg("-i")
        .arg("1s")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--interval <SEC>'",
        ));
    utils::command::command()
        .arg("-q")
        .arg("-N")
        .arg("foo")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--quiet' cannot be used with '--name <NAME>'",
        ));
}
