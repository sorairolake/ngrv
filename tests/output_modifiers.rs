// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn si() {
    command::command()
        .arg("-k")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn size() {
    command::command()
        .arg("-s")
        .arg("128MiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn size_with_invalid_byte() {
    command::command()
        .arg("-s")
        .arg("2048 A")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    command::command()
        .arg("-s")
        .arg("2.00LiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
    command::command()
        .arg("-s")
        .arg("n B")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    command::command()
        .arg("-s")
        .arg("n")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    command::command()
        .arg("-s")
        .arg("nKiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
}

#[test]
fn interval() {
    command::command()
        .arg("-i")
        .arg("1s")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn interval_with_invalid_span() {
    command::command()
        .arg("-i")
        .arg("NaN")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse input in the "friendly" duration format"#,
        ));
    command::command()
        .arg("-i")
        .arg("1")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse input in the "friendly" duration format"#,
        ));
    command::command()
        .arg("-i")
        .arg("1a")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse input in the "friendly" duration format"#,
        ));
    command::command()
        .arg("-i")
        .arg("10000000000000y")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse input in the "friendly" duration format"#,
        ));
}

#[test]
fn name() {
    command::command()
        .arg("-N")
        .arg("foo")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn bar_style() {
    command::command()
        .arg("-u")
        .arg("#>-")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn spinner_style() {
    command::command()
        .arg("--spinner-style")
        .arg(r"/|\- ")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
    command::command()
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
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn spinner_style_without_value() {
    command::command()
        .arg("--spinner-style")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "a value is required for '--spinner-style <STRING>...' but none was supplied",
        ));
}
