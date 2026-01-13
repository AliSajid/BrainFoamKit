<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: CC0-1.0
-->

# BrainFoamKit AI Coding Instructions

## Project Overview

BrainFoamKit is a Brainfuck interpreter and TUI visualizer written in Rust. The project consists of:

- **Core library** (`brainfoamkit_lib`): Virtual machine, instruction set, and low-level types
- **Interpreter** (`bfkrun`): CLI tool to execute Brainfuck programs
- **Visualizer** (`bfkview`): TUI for stepping through programs (using `ratatui`)
- **Documentation** (`guide/`): mdbook-based developer guide

### Current Status: 🟡 MVP Development (v1.1 Milestone)

**What Works:**

- ✅ Custom type system (Bit, Nybble, Byte) with full trait implementations
- ✅ VirtualMachine architecture with builder pattern
- ✅ VMReader trait for generic input
- ✅ Basic VM operations (pointer movement, increment/decrement)
- ✅ Professional CI/CD infrastructure
- ✅ SPDX licensing compliance

**Critical Blockers (Cannot Execute Real BF Programs):**

- ❌ Loop control (`[` and `]` instructions) - see [#414](https://github.com/AliSajid/BrainFoamKit/issues/414)
- ❌ Output (`.` instruction) - see [#415](https://github.com/AliSajid/BrainFoamKit/issues/415)
- ❌ Functional CLI (`bfkrun` doesn't execute .bf files) - see [#418](https://github.com/AliSajid/BrainFoamKit/issues/418)
- ❌ TUI debugger (`bfkview` shows placeholder) - see [#420](https://github.com/AliSajid/BrainFoamKit/issues/419)

**IMPORTANT:** Focus on issues tagged `blocks-mvp` before working on enhancements or refactoring.

## Architecture & Key Components

### Core Type System (src/brainfoamkit_lib/)

The project builds from bit-level primitives upward:

- `Bit` → `Nybble` (4 bits) → `Byte` (8 bits) - Custom types with comprehensive bitwise operations
- `Instruction` enum: Maps 8 Brainfuck commands (`>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`) plus `NoOp`
- `Program`: Validated instruction sequences
- `VirtualMachine<R>`: Generic over `VMReader` trait for input handling

### Builder Pattern

Use `VirtualMachineBuilder` for construction - never construct `VirtualMachine` directly:

```rust
VirtualMachine::builder()
    .tape_size(1024)
    .program(program)
    .input_device(stdin)
    .build()
```

### Test Organization

All modules include inline `#[cfg(test)]` blocks. Tests live alongside implementation code, not in separate `tests/` directory.

## Development Workflows

### Build & Check

- **Primary tool**: `bacon` (configured in `bacon.toml`) - runs clippy with strict lints on save
- Run task: "Run bacon" or `bacon` in terminal
- Default job: `clippy` with nightly + pedantic/nursery lints
- Available jobs: `check`, `check-all`, `test`, `test-nextest`, `doc`, `clippy-all-targets`

### Testing

- **Test runner**: `cargo nextest` (configured in `nextest.toml`)
- Configuration: `test-threads = "num-cpus"`, shows all test statuses including skipped
- Run: `cargo nextest run` or `bacon test-nextest`

### Strict Linting

`clippy` configuration enforces:

- `-W clippy::pedantic`, `-W clippy::nursery`
- `-W clippy::unwrap_used`, `-W clippy::expect_used`
- No `unwrap()` or `expect()` allowed - use `anyhow::Result` for error handling
- Suppress specific lints only with team approval

### Code Formatting

- Use `rustfmt` and `clippy` - checked in CI
- Follow Conventional Commits for commit messages

## Project Conventions

### Licensing & SPDX

**CRITICAL**: Every file must include SPDX headers:

```rust
// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
```

Dual-licensed under MIT OR Apache-2.0. Configuration and other ancillary files use `CC0-1.0`.

### Documentation Standards

- Comprehensive doc comments with `# Examples` sections
- Reference the [grammar](lang/grammar.ebnf) and [language docs](guide/src/language_reference/)
- Link to GitHub repo in docs: `https://github.com/AliSajid/brainfoamkit`

### Minimum Supported Rust Version (MSRV)

- **1.86.0** as defined in `Cargo.toml` and tested in CI
- Edition 2024

## CI/CD & Automation

### Continuous Integration

- Matrix testing: stable/beta/nightly/MSRV across Linux/Windows/macos
- Path filters skip source builds when only docs change
- Uses `cargo-deny` for dependency auditing (`deny.toml`)
- Gist-based build badges (see `README.md`)

### Container Support

- Dockerfile available at project root
- Container builds in `.github/workflows/build_container.yaml`

### Release Process

- Semantic versioning with pre-release tags (e.g., `1.1.1-next.12`)
- Automated via `.github/workflows/release.yaml`
- Changelog generation from Conventional Commits

## Key Files & Patterns

### Entry Points

- [src/brainfoamkit_interpreter/main.rs](../src/brainfoamkit_interpreter/main.rs): ASCII table printer demo
- [src/brainfoamkit_visualizer/main.rs](../src/brainfoamkit_visualizer/main.rs): TUI app with terminal setup/teardown pattern

### Critical Implementation Files

- [src/brainfoamkit_lib/machine.rs](../src/brainfoamkit_lib/machine.rs): VM execution logic (737 lines)
- [src/brainfoamkit_lib/instruction.rs](../src/brainfoamkit_lib/instruction.rs): Instruction enum with `from_char()` parser
- [src/brainfoamkit_lib/byte.rs](../src/brainfoamkit_lib/byte.rs): Extensive bitwise operations (1964 lines)

### Configuration

- [bacon.toml](../bacon.toml): Watch mode with nightly clippy
- [nextest.toml](../nextest.toml): Test execution settings
- [deny.toml](../deny.toml): Dependency policy for 10 target triples
- [about.toml](../about.toml): License report generation

## Common Tasks

### Priority Order for Contributors

#### **Week 1-2: MVP Critical Path** (Work on these FIRST)

1. **Loop Control** ([#414](https://github.com/AliSajid/BrainFoamKit/issues/414)) - Implement `jump_forward()` and `jump_backward()`
2. **Output Value** ([#415](https://github.com/AliSajid/BrainFoamKit/issues/415)) - Implement `.` instruction
3. **Pointer Wrapping** ([#416](https://github.com/AliSajid/BrainFoamKit/issues/416)) - Fix boundary wrapping
4. **Functional CLI** ([#418](https://github.com/AliSajid/BrainFoamKit/issues/418)) - Make `bfkrun` execute .bf files
5. **Integration Tests** ([#417](https://github.com/AliSajid/BrainFoamKit/issues/417)) - End-to-end test coverage

#### **Post-MVP: Enhancements**

- VMWriter trait ([#420](https://github.com/AliSajid/BrainFoamKit/issues/420))
- TUI debugger ([#419](https://github.com/AliSajid/BrainFoamKit/issues/419))
- Performance benchmarks ([#422](https://github.com/AliSajid/BrainFoamKit/issues/422))
- Documentation audit ([#421](https://github.com/AliSajid/BrainFoamKit/issues/421))
- Macro refactoring ([#423](https://github.com/AliSajid/BrainFoamKit/issues/423))

### Quick Reference

**Add new instruction**: Update `Instruction` enum → `from_char()` → `VirtualMachine::execute_instruction()` → tests
**Modify VM behavior**: Use builder pattern, never mutate tape directly without helper methods
**Add dependencies**: Check `cargo-deny` policy first, justify in PR
**Update docs**: Edit `guide/src/` markdown, rebuild with `mdbook build guide`

### Known Architectural Decisions

**Custom Byte Type vs Native u8:**

- Current: Custom `Byte` type with 8 `Bit` fields
- Pros: Educational, type-safe, explicit operations
- Cons: Estimated 100-1000x slower than native u8
- Decision: Keep for now, benchmark in [#422](https://github.com/AliSajid/BrainFoamKit/issues/422), revisit post-MVP

**No Integration Tests:**

- Tracked in [#417](https://github.com/AliSajid/BrainFoamKit/issues/417)
- Reason: Core functionality incomplete
- Plan: Add after MVP features work

**Incomplete VM Methods:**

- `jump_forward()`, `jump_backward()`, `output_value()` all use `todo!()`
- This is intentional - implementation is the MVP focus
- See [#414](https://github.com/AliSajid/BrainFoamKit/issues/414) and [#415](https://github.com/AliSajid/BrainFoamKit/issues/415)

## Issue Management

### Milestone: v1.1 (MVP)

**Goal:** Make BrainFoamKit actually interpret Brainfuck programs

**Tracking:** [Epic #424](https://github.com/AliSajid/BrainFoamKit/issues/424)

**Timeline:** 3 weeks from issue creation

### Issue Labels

- `priority: critical` - Blocks MVP, work on FIRST
- `blocks-mvp` - Cannot ship v1.1 without this
- `component: vm` - Virtual machine core
- `component: cli` - Command-line interface
- `component: tui` - Terminal UI
- `type: bug` - Something broken
- `type: feature` - New functionality
- `type: refactor` - Code quality improvement

### Before Starting Work

1. Check if issue is tagged `blocks-mvp` - prioritize these
2. Read the issue description and acceptance criteria
3. Check linked dependencies (e.g., #414 blocks #415)
4. Review referenced code locations in the issue
5. Run `bacon clippy` to ensure clean starting point
