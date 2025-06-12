// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

#[test]
fn si() {
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
        .arg("-s")
        .arg("2048 A")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    utils::command::command()
        .arg("-s")
        .arg("2.00LiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
    utils::command::command()
        .arg("-s")
        .arg("n B")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
        .arg("-s")
        .arg("n")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
        .arg("-i")
        .arg("NaN")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "NaN" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("1")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "1" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("1a")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            r#"failed to parse "1a" in the "friendly" format"#,
        ));
    utils::command::command()
        .arg("-i")
        .arg("10000000000000y")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
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
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn bar_style() {
    utils::command::command()
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
    utils::command::command()
        .arg("--spinner-style")
        .arg(r"/|\- ")
        .arg("--")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
    utils::command::command()
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
    utils::command::command()
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
