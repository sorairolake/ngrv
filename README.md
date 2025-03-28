<!--
SPDX-FileCopyrightText: 2025 Shun Sakai

SPDX-License-Identifier: GPL-3.0-or-later
-->

# ngrv

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
![License][license-badge]

**ngrv** (*n*a*g*a*r*e *v*iewer, 流れビューアー) is a terminal-based tool for
monitoring the progress of data through a pipeline written in [Rust]. It is
inspired by [`pv(1)`] and can be used for similar purposes, but is not intended
as a drop-in replacement for it.

![Demo animation](assets/demo.gif)

## Installation

### From source

```sh
cargo install ngrv
```

### From binaries

The [release page] contains pre-built binaries for Linux, macOS and Windows.

### How to build

Please see [BUILD.adoc].

## Usage

### Basic usage

To watch the progress of compressing a `.gz` file:

```sh
ngrv archive.tar | gzip > archive.tar.gz
```

A similar example that read a `.tar` archive from standard input:

```sh
cat archive.tar | ngrv -s 4GiB | gzip > archive.tar.gz
```

> [!TIP]
> If any inputs are not files or the size of the file cannot be queried, the
> progress bar will not be shown unless `-s` is specified.

### Generate shell completion

`--generate-completion` option generates shell completions to standard output.

The following shells are supported:

- `bash`
- `elvish`
- `fish`
- `nushell`
- `powershell`
- `zsh`

Example:

```sh
ngrv --generate-completion bash > ngrv.bash
```

## Command-line options

Please see the following:

- [`ngrv(1)`]

## Source code

The upstream repository is available at
<https://github.com/sorairolake/ngrv.git>.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## Acknowledgment

This program is inspired by [`pv`].

## License

Copyright (C) 2025 Shun Sakai (see [AUTHORS.adoc])

1.  This program is distributed under the terms of the _GNU General Public
    License v3.0 or later_.
2.  Some files are distributed under the terms of the _Creative Commons
    Attribution 4.0 International Public License_.

This project is compliant with version 3.3 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/ngrv/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/ngrv/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/ngrv?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/ngrv
[msrv-badge]: https://img.shields.io/crates/msrv/ngrv?style=for-the-badge&logo=rust
[license-badge]: https://img.shields.io/crates/l/ngrv?style=for-the-badge
[Rust]: https://www.rust-lang.org/
[`pv(1)`]: https://www.ivarch.com/programs/quickref/pv.shtml
[release page]: https://github.com/sorairolake/ngrv/releases
[BUILD.adoc]: BUILD.adoc
[`ngrv(1)`]: docs/man/man1/ngrv.1.adoc
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[`pv`]: https://www.ivarch.com/programs/pv.shtml
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec-3.3/
