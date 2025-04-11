// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn buffer_size() {
    utils::command::command()
        .arg("-B")
        .arg("128MiB")
        .arg("assets/after-long-help.md")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("assets/after-long-help.md")));
}

#[test]
fn buffer_size_with_invalid_byte() {
    utils::command::command()
        .arg("-B")
        .arg("2048 A")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    utils::command::command()
        .arg("-B")
        .arg("2.00LiB")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
    utils::command::command()
        .arg("-B")
        .arg("n B")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("-B")
        .arg("n")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("-B")
        .arg("nKiB")
        .arg("assets/after-long-help.md")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
}
