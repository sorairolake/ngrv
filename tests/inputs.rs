// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn inputs() {
    utils::command::command()
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
    utils::command::command()
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .arg("data/LICENSES/CC-BY-4.0.txt")
        .assert()
        .success()
        .stdout(predicate::eq(concat!(
            include_str!("data/LICENSES/GPL-3.0-or-later.txt"),
            include_str!("data/LICENSES/CC-BY-4.0.txt")
        )));
}

#[test]
fn inputs_from_stdin() {
    utils::command::command()
        .write_stdin(include_str!("data/LICENSES/GPL-3.0-or-later.txt"))
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
    utils::command::command()
        .arg("-")
        .write_stdin(include_str!("data/LICENSES/GPL-3.0-or-later.txt"))
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
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
            .arg("data/LICENSES/CC-BY-4.0.txt")
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
            .arg("data/LICENSES/GPL-3.0-or-later.txt")
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
