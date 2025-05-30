# SPDX-FileCopyrightText: 2025 Shun Sakai
#
# SPDX-License-Identifier: GPL-3.0-or-later

alias lint := clippy

# Run default recipe
_default:
    just -l

# Build a package
build:
    cargo build

# Remove generated artifacts
clean:
    cargo clean

# Check a package
check:
    cargo check

# Run tests
test:
    cargo test

# Run the formatter
fmt:
    cargo fmt

# Run the formatter with options
fmt-with-options:
    cargo +nightly fmt

# Run the linter
clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

# Run the linter for GitHub Actions workflow files
lint-github-actions:
    actionlint -verbose

# Run the formatter for the README
fmt-readme:
    npx prettier -w README.md

# Increment the version
bump part:
    bump-my-version bump {{ part }}
    cargo set-version --bump {{ part }}
