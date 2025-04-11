// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn si() {
    utils::command::command()
        .arg("-k")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn size() {
    utils::command::command()
        .arg("-s")
        .arg("128MiB")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn size_with_invalid_byte() {
    utils::command::command()
        .arg("-s")
        .arg("2048 A")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    utils::command::command()
        .arg("-s")
        .arg("2.00LiB")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
    utils::command::command()
        .arg("-s")
        .arg("n B")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("-s")
        .arg("n")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("-s")
        .arg("nKiB")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
}

#[test]
fn interval() {
    utils::command::command()
        .arg("-i")
        .arg("1s")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn interval_with_invalid_span() {
    utils::command::command()
        .arg("-i")
        .arg("NaN")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "NaN" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("1")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "1" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("1a")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "1a" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("10000000000000y")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "10000000000000y" in the "friendly" format"#,
        ));
}

#[test]
fn name() {
    utils::command::command()
        .arg("-N")
        .arg("foo")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}
