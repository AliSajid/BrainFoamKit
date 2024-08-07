<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: CC0-1.0
-->

# Brainfoamkit

![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/brainfoamkit)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/brainfoamkit)
[![Continuous integration](https://github.com/AliSajid/BrainFoamKit/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/BrainFoamKit/actions/workflows/ci.yaml)
[![REUSE status](https://api.reuse.software/badge/github.com/AliSajid/brainfoamkit)](https://api.reuse.software/info/github.com/AliSajid/brainfoamkit)

This project aims to implement a [brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter in Rust. In addition, this
also aims to provide a terminal-based TUI for stepping through and understanding the brainfuck programs.

## Builds

|         | Stable | Beta | Nightly | MSRV (1.75.0) |
| ------- | ------ | ---- | ------- | ---- |
| Linux   | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/ubuntu-stable.json) | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/ubuntu-beta.json) | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/ubuntu-nightly.json) | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/ubuntu-msrv.json) |
| Windows  | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/windows-stable.json) | ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/windows-beta.json) | ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/windows-nightly.json) |![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/windows-msrv.json) |
| macos   | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/macos-stable.json) | ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/macos-beta.json) | ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/macos-nightly.json) | ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/80eb42183fabbaf02eebcf768bdae485/raw/macos-msrv.json) |

## Rationale

Brainfuck is an interesting esoteric language. It is turing complete and can essentially be used as a general purpose
programming language. However, it has only 8 individual symbols that are used to instruct the interpreter. This makes it
both fun and challenging to implement.

While several C and C++ interpreters for Brainfuck exist, I believe that it is particularly well suited for
implementation in Rust due the combination of memory-safety, speed and zero-cost abstractions. Additionally, the
interpreter is expected to be non-trivial in complexity while still only scratching the surface of the features Rust has
to offer. Thus, it provides an excellent educational opportunity for someone trying to learn Rust.

## Details

The major details for the project are available in the [BrainFoamKit Book](https://brainfoamkit.imamiland.com/).

## (Planned) Features

- A complete brainfuck interpreter capable of ingesting a brainfuck program and behaving appropriately
- A modular system for the said interpreter, allowing for extensions and modifications
- A configurable brainfuck virtual machine to interpret the programs.
- A fully capable TUI to visualize and step through a brainfuck program

## Current Status

- Implement basic building blocks for the Virtual Machine
- Implement the Virtual Machine to run the code
- Implement a parser for parsing the input program
- Design the TUI for the visualizer
- Implement the TUI with the Virtual Machine and parser

## Contributing

See the [Contributing](CONTRIBUTING.md) for details on how to contribute to the project.

You can contribute to the project through [GitPod](https://gitpod.io).

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#AliSajid/brainfoamkit)

## Code of Conduct

This project is governed by the Contributor Code of Conduct Covenant. Details are outlined in
the [CODE OF CONDUCT](CODE_OF_CONDUCT.md).
