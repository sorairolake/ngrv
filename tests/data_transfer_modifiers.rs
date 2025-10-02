// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn buffer_size() {
    command::command()
        .arg("-B")
        .arg("128MiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/LICENSES/GPL-3.0-or-later.txt"
        )));
}

#[test]
fn buffer_size_with_invalid_byte() {
    command::command()
        .arg("-B")
        .arg("2048 A")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'A' is incorrect"));
    command::command()
        .arg("-B")
        .arg("2.00LiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("the character 'L' is incorrect"));
    command::command()
        .arg("-B")
        .arg("n B")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    command::command()
        .arg("-B")
        .arg("n")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
    command::command()
        .arg("-B")
        .arg("nKiB")
        .arg("data/LICENSES/GPL-3.0-or-later.txt")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the character 'n' is not a number",
        ));
}
