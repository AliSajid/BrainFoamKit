# Introduction

![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/brainfoamkit)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/brainfoamkit)
[![Continuous integration](https://github.com/AliSajid/BrainFoamKit/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/BrainFoamKit/actions/workflows/ci.yaml)

`BrainFoamKit`, or `BFK` as I like to call it, is a project that aims to implement a [brainf**k, (or BF)](https://esolangs.org/wiki/Brainfuck) interpreter in Rust. To make things easier for myself, and to keep up with the times, I have added one more instruction However, it's not JUST an interpreter. The broader scope aim of this project is to implement a complete, if poorly designed, and functional Virtual Machine (the _BFKVM_) that can interpret the instructions and allow for us to take a cross-section look as it executes the `BFK` program.

For this purpose, the project has three distinct parts:

1. `brainfoamkit_lib`, a library that implements the `BFKVM` and the interpreter for `BFK`.
2. `bfkrun`, a binary that can be used to run programs written in either `BF` or `BFK` dialects.
3. `bfkview`, a TUI application with a (hopefully) intutive interface that lets you step through BF[K] programs.

## Rationale

Brainf**k is an interesting esoteric language. It is turing complete and can theoretically be used as a general purpose programming language. However, it has only 8 individual symbols that are used to instruct the interpreter. This makes it both fun and challenging to implement.

While several C and C++ interpreters for Brainf**k exist, I believe that it is particularly well suited for
implementation in Rust due the combination of memory-safety, speed and zero-cost abstractions. Additionally, the
interpreter is expected to be non-trivial in complexity while still only scratching the surface of the features Rust has to offer. Thus, it provides an excellent educational opportunity for someone trying to learn Rust.

## Implementation Details

Briefly, the interpreter is implemented as a Virtual Machine on top of the existing Rust runtime. This frees us from
low-level hardware constraints, allowing us to focus on the core of the program. The language remains focused on `Byte`-level operations and the ASCII code.

More details regarding the implementation, including an EBNF grammar and other design tradeoffs can be found in
the language reference pages for the [original BF Specification](./language_reference/README.md) and the [`BFK` dialect](./language_changes/README.md).

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

Progress for the project can be tracked on the [Github Issues](https://github.com/AliSajid/BrainFoamKit/issues) and
the [Github Project](https://github.com/users/AliSajid/projects/1) pages.

## Contributing

See the [Contributing](CONTRIBUTING.md) for details on how to contribute to the project.

You can contribute to the project through [GitPod](https://gitpod.io).

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#AliSajid/brainfoamkit)

## Code of Conduct

This project is governed by the Contributor Code of Conduct Covenant. Details are outlined in
the [CODE OF CONDUCT](CODE_OF_CONDUCT.md).
