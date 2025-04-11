// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn inputs() {
    utils::command::command()
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
    utils::command::command()
        .arg("assets/after-long-help.md")
        .arg("assets/long-version.md")
        .assert()
        .success()
        .stdout(predicate::eq(concat!(
            include_str!("assets/after-long-help.md"),
            include_str!("assets/long-version.md")
        )));
}

#[test]
fn inputs_from_stdin() {
    utils::command::command()
        .write_stdin(include_str!("assets/after-long-help.md"))
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
    utils::command::command()
        .arg("-")
        .write_stdin(include_str!("assets/after-long-help.md"))
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn inputs_from_non_existent_files() {
    {
        let command = utils::command::command()
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66)
            .stderr(predicate::str::contains("could not open non_existent.txt"));
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                "The system cannot find the file specified. (os error 2)",
            ));
        } else {
            command.stderr(predicate::str::contains(
                "No such file or directory (os error 2)",
            ));
        }
    }
    {
        let command = utils::command::command()
            .arg("non_existent.txt")
            .arg("assets/long-version.md")
            .assert()
            .failure()
            .code(66)
            .stderr(predicate::str::contains("could not open non_existent.txt"));
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                "The system cannot find the file specified. (os error 2)",
            ));
        } else {
            command.stderr(predicate::str::contains(
                "No such file or directory (os error 2)",
            ));
        }
    }
    {
        let command = utils::command::command()
            .arg("assets/after-long-help.md")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66)
            .stderr(predicate::str::contains("could not open non_existent.txt"));
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                "The system cannot find the file specified. (os error 2)",
            ));
        } else {
            command.stderr(predicate::str::contains(
                "No such file or directory (os error 2)",
            ));
        }
    }
}
