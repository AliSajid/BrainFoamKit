<!--
SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami

SPDX-License-Identifier: CC0-1.0
-->

<!-- omit in toc -->
# Contributing to Brainfoamkit

First off, thanks for taking the time to contribute!

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions.

And if you like the project, but just don't have time to contribute, that's fine. You can show your appreciation and support for this project in other easy ways  which we would also be very happy about:

- Star the project
- Tweet about it
- Cite the project in your publications if you found it helpful
- Refer this project in your project's [README](README)
- Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
    - [Prerequisites](#prerequisites)
    - [Getting Started](#getting-started)
    - [Development Workflow](#development-workflow)
      - [**Manual Testing**](#manual-testing)
      - [**Building Binaries**](#building-binaries)
      - [**Documentation**](#documentation)
    - [Repository Structure](#repository-structure)
    - [What to Work On](#what-to-work-on)
      - [**Check the v1.1 Milestone First!**](#check-the-v11-milestone-first)
      - [**Priority Order:**](#priority-order)
      - [**Finding Good First Issues:**](#finding-good-first-issues)
    - [Making Changes](#making-changes)
    - [Pull Request Guidelines](#pull-request-guidelines)
    - [Testing Your Changes](#testing-your-changes)
    - [Troubleshooting](#troubleshooting)
      - [**"clippy is too strict!"**](#clippy-is-too-strict)
      - [**"Tests are slow"**](#tests-are-slow)
      - [**"bacon keeps failing"**](#bacon-keeps-failing)
      - [**"SPDX headers missing"**](#spdx-headers-missing)
      - [**"Can't find artifacts/ directory"**](#cant-find-artifacts-directory)
    - [Getting Help](#getting-help)
  - [Improving The Documentation](#improving-the-documentation)
    - [API Documentation (Rust Docs)](#api-documentation-rust-docs)
    - [User Guide (mdbook)](#user-guide-mdbook)
    - [Copilot Instructions](#copilot-instructions)
    - [README and Top-Level Docs](#readme-and-top-level-docs)
    - [Language Specification](#language-specification)
- [Style Guides](#style-guides)
  - [Rust Code Style](#rust-code-style)
    - [Formatting with rustfmt](#formatting-with-rustfmt)
    - [Linting with Clippy](#linting-with-clippy)
    - [Architecture Patterns](#architecture-patterns)
    - [Documentation Style](#documentation-style)
    - [SPDX License Headers](#spdx-license-headers)
    - [File Organization](#file-organization)
- [Automation Tools and Scripts](#automation-tools-and-scripts)
  - [Build Automation](#build-automation)
    - [bacon (Watch Mode)](#bacon-watch-mode)
    - [nextest (Test Runner)](#nextest-test-runner)
  - [Release Automation](#release-automation)
    - [scripts/build.sh](#scriptsbuildsh)
    - [scripts/generate\_about\_\*.sh](#scriptsgenerate_about_sh)
  - [Dependency Management](#dependency-management)
    - [cargo-deny](#cargo-deny)
  - [Code Quality](#code-quality)
    - [Pre-commit Hooks](#pre-commit-hooks)
    - [REUSE SPDX Compliance](#reuse-spdx-compliance)
  - [CI/CD Workflows](#cicd-workflows)
  - [Commit Messages](#commit-messages)
    - [Format](#format)
    - [Types](#types)
    - [Scopes](#scopes)
    - [Examples](#examples)
    - [Guidelines](#guidelines)
    - [Commit Often](#commit-often)
    - [Amending Commits](#amending-commits)
- [Join The Project Team](#join-the-project-team)

<!-- omit in toc -->
## Quick Reference

**New Contributors:**

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/BrainFoamKit.git
cd BrainFoamKit

# 2. Install `mise`
curl https://mise.run | sh

Then run `mise install`

# 3. Start bacon (continuous checking)
bacon

# 4. Run tests
cargo nextest run

# 5. Make changes, commit, and push
git switch --create feat/my-feature
git commit -m "feat(vm): add my feature"
git push origin feat/my-feature
```

**Daily Workflow:**

- Run `bacon` in terminal (auto-checks on save)
- Write code with SPDX headers (see [SPDX License Headers](#spdx-license-headers))
- Test with `cargo nextest run`
- Commit with [Conventional Commits](#commit-messages) format
- Check [Milestone v1.1](https://github.com/AliSajid/BrainFoamKit/milestone/1) for priority work

**Current Focus: MVP Features ([Issue #424](https://github.com/AliSajid/BrainFoamKit/issues/424))**

- [#414](https://github.com/AliSajid/BrainFoamKit/issues/414) - Loop control (`[` and `]`)
- [#415](https://github.com/AliSajid/BrainFoamKit/issues/415) - Output value (`.`)
- [#416](https://github.com/AliSajid/BrainFoamKit/issues/416) - Pointer wrapping
- [#418](https://github.com/AliSajid/BrainFoamKit/issues/418) - Functional CLI

**Key Resources:**

- [mise](https://mise.jdx.dev/) - Tool version manager (manages Rust, bacon, nextest, and all dev tools)
- [`.github/copilot-instructions.md`](.github/copilot-instructions.md) - Architecture and patterns
- [`bacon.toml`](bacon.toml) - Watch mode configuration
- [`nextest.toml`](nextest.toml) - Test runner settings
- [`deny.toml`](deny.toml) - Dependency policies
- [User Guide](https://alisajid.github.io/BrainFoamKit/) - Full documentation

---


## I Have a Question

If you want to ask a question, we assume that you have read the available [Documentation](https://docs.rs/brainfoamkit).

Before you ask a question, it is best to search for existing [Issues](https://github.com/AliSajid/brainfoamkit/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/AliSajid/brainfoamkit/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions depending on what seems relevant.

We will then take care of the issue as soon as possible.

## I Want To Contribute

### Legal Notice <!-- omit in toc -->

When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.


This project is dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE). When you submit changes, your submissions are understood to be under the same [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) that covers the project. Feel free to contact the maintainers if that's a concern.


### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://docs.rs/brainfoamkit). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/AliSajid/brainfoamkit/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback). We use RUST_BACKTRACE=1 to get a full stack trace.
  - OS, Platform and Version (Windows, Linux, macos, x86, ARM)
  - Version of Rust, Cargo, and other environment components if applicable
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public.
We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/AliSajid/brainfoamkit/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).


### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for Gainful Key, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://docs.rs/brainfoamkit) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/AliSajid/brainfoamkit/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/AliSajid/brainfoamkit/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macos and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most Gainful Key users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

### Your First Code Contribution

Welcome! Here's how to get started with BrainFoamKit development.

#### Prerequisites

- [mise](https://mise.jdx.dev/) - Development tool manager (installs Rust, bacon, nextest, and other tools)
- Git with GPG signing configured (optional but recommended)

**Installing mise:**

```bash
# macOS/Linux
curl https://mise.run | sh

# Or via Homebrew
brew install mise
```

Once mise is installed, all project tools will be automatically managed.

#### Getting Started

1. **Fork and Clone**

   ```bash
   git clone https://github.com/YOUR_USERNAME/BrainFoamKit.git
   cd BrainFoamKit
   ```

2. **Install Development Tools**

   ```bash
   # Install all tools (Rust, bacon, nextest, etc.) via mise
   mise install

   # Install pre-commit hooks
   mise run install-hooks
   ```

3. **Verify Setup**

   ```bash
   # Run clippy checks
   bacon clippy

   # Run tests
   cargo nextest run

   # Build all binaries
   cargo build --all-targets
   ```

#### Development Workflow

**Watch Mode Development** (Recommended)

Start `bacon` in your terminal - it will watch for file changes and run clippy automatically:

```bash
bacon          # Default: runs clippy with strict lints
bacon test     # Runs tests on file changes
bacon check    # Runs cargo check
```

See `bacon.toml` for all available jobs and keybindings (`c` for clippy, `t` for test, etc.).

##### **Manual Testing**

```bash
# Run all tests with nextest (faster)
cargo nextest run

# Run specific test
cargo nextest run test_increment_pointer

# Run with backtrace
RUST_BACKTRACE=1 cargo nextest run
```

##### **Building Binaries**

```bash
# Build both bfkrun and bfkview
cargo build --release

# Run the interpreter (currently shows ASCII table)
cargo run --bin bfkrun

# Run the TUI visualizer (currently placeholder)
cargo run --bin bfkview
```

##### **Documentation**

```bash
# Build the mdBook guide
cd guide
mdbook build
mdbook serve  # View at http://localhost:3000

# Generate Rust API docs
cargo doc --no-deps --open
```

#### Repository Structure

```bash
BrainFoamKit/
├── .github/
│   ├── workflows/           # CI/CD workflows
│   │   ├── ci.yaml         # Main CI (builds, tests, lints)
│   │   ├── release.yaml    # Semantic release automation
│   │   └── deploy_mdbook.yaml
│   └── copilot-instructions.md  # AI coding assistant guide
├── src/
│   ├── brainfoamkit_lib/   # Core library
│   │   ├── machine.rs      # VirtualMachine implementation
│   │   ├── instruction.rs  # BF instruction enum
│   │   ├── byte.rs         # Custom Byte type (1964 lines!)
│   │   └── vm_reader.rs    # Input abstraction trait
│   ├── brainfoamkit_interpreter/  # bfkrun CLI
│   └── brainfoamkit_visualizer/   # bfkview TUI
├── scripts/
│   ├── build.sh            # Release artifact preparation
│   ├── generate_about_json.sh
│   └── generate_about_md.sh
├── guide/                  # mdBook documentation
├── bacon.toml             # Watch mode configuration
├── nextest.toml           # Test runner settings
├── deny.toml              # Dependency policy
└── about.toml             # License compliance

```

#### What to Work On

##### **Check the v1.1 Milestone First!**

We're currently focused on MVP functionality. See [Issue #424](https://github.com/AliSajid/BrainFoamKit/issues/424) for the roadmap.

##### **Priority Order:**

1. **Critical (Week 1-2)** - Issues tagged `blocks-mvp`:
   - [#414](https://github.com/AliSajid/BrainFoamKit/issues/414) - Loop control (`[` and `]`)
   - [#415](https://github.com/AliSajid/BrainFoamKit/issues/415) - Output value (`.`)
   - [#416](https://github.com/AliSajid/BrainFoamKit/issues/416) - Pointer wrapping
   - [#418](https://github.com/AliSajid/BrainFoamKit/issues/418) - Functional CLI

2. **High Priority:**
   - [#417](https://github.com/AliSajid/BrainFoamKit/issues/417) - Integration tests

3. **Post-MVP:**
   - [#419](https://github.com/AliSajid/BrainFoamKit/issues/419) - TUI debugger
   - [#420](https://github.com/AliSajid/BrainFoamKit/issues/420) - VMWriter trait
   - [#421-423](https://github.com/AliSajid/BrainFoamKit/issues/421) - Documentation, benchmarks, refactoring

##### **Finding Good First Issues:**

- Look for issues tagged `good-first-issue`
- Check `component: cli` or `component: tui` for less complex work
- Avoid `component: vm` and `component: primitives` until familiar with the codebase

#### Making Changes

1. **Create a Feature Branch**

   ```bash
   # Follow convention: type/description
   git checkout -b feat/loop-control
   git checkout -b fix/pointer-wrapping
   git checkout -b docs/api-examples
   ```

2. **Write Code Following Standards**
   - **SPDX Headers**: Every file must start with:

     ```rust
     // SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
     //
     // SPDX-License-Identifier: Apache-2.0
     // SPDX-License-Identifier: MIT
     ```

   - **No `unwrap()`/`expect()`**: Use `anyhow::Result` for error handling
   - **Inline Tests**: Add `#[cfg(test)]` modules in the same file
   - **Doc Comments**: Include `# Examples` sections in public APIs
   - **Builder Pattern**: Use `VirtualMachine::builder()` - never construct directly

3. **Test Your Changes**

   ```bash
   # Run clippy (catches most issues)
   bacon clippy

   # Run tests
   cargo nextest run

   # Check all targets
   cargo clippy --all-targets
   ```

4. **Update Documentation**
   - Add/update doc comments
   - Update `guide/` if changing public API
   - Update `CHANGELOG.md` is auto-generated - commit messages matter!

5. **Commit with Conventional Commits**

   ```bash
   # Format: <type>(<scope>): <description>
   git commit -m "feat(vm): implement loop control logic"
   git commit -m "fix(cli): handle missing input files"
   git commit -m "docs(guide): add VM architecture diagram"
   ```

   **Types:** `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `ci`, `perf`
   **Scopes:** `vm`, `cli`, `tui`, `primitives`, `ci`, `deps`

6. **Push and Create PR**

   ```bash
   git push origin feat/loop-control
   ```

   Then open a PR on GitHub targeting the `main` branch.

#### Pull Request Guidelines

**PR Title:** Use Conventional Commits format (e.g., `feat(vm): add jump_forward implementation`)

**PR Description Should Include:**

- Link to the issue(s) being addressed (e.g., "Closes #414")
- Summary of changes made
- Testing performed
- Screenshots/recordings for UI changes

**CI Must Pass:**

- ✅ All tests pass on stable/beta/nightly/MSRV
- ✅ Clippy with pedantic/nursery lints (no warnings)
- ✅ `rustfmt` formatting check
- ✅ cargo-deny license compliance
- ✅ REUSE SPDX compliance

**Review Process:**

- Maintainers will review within 3-5 days
- Address feedback by pushing new commits
- Once approved, maintainers will merge (squash merge)

#### Testing Your Changes

**Unit Tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_pointer() {
        let mut vm = VirtualMachine::builder().build().unwrap();
        vm.increment_pointer();
        assert_eq!(vm.memory_pointer(), 1);
    }
}
```

**Integration Tests** (coming in [#417](https://github.com/AliSajid/BrainFoamKit/issues/417)):

```bash
# Will be added to tests/ directory
cargo nextest run --test integration_test
```

**Manual Testing:**

```bash
# Test the CLI (after #418 is done)
cargo run --bin bfkrun -- examples/hello_world.bf

# Test the TUI
cargo run --bin bfkview
```

#### Troubleshooting

##### **"clippy is too strict!"**

- That's intentional. We use `-W clippy::pedantic` and `-W clippy::nursery`
- If you think a lint is wrong, discuss with maintainers before suppressing
- Never use `#[allow(clippy::unwrap_used)]` - use proper error handling

##### **"Tests are slow"**

- Use `cargo nextest run` instead of `cargo test` (configured for parallel execution)
- Run specific tests: `cargo nextest run test_name`

##### **"bacon keeps failing"**

- Check `bacon.toml` for the current job configuration
- Try `bacon check` for basic compile checks first
- Ensure you're on Rust stable (or the version specified in the error)

##### **"SPDX headers missing"**

- CI checks with `reuse lint`
- Add headers to every new file (see template above)
- Use `CC0-1.0` for configuration files, MIT/Apache-2.0 for code

##### **"Can't find artifacts/ directory"**

- This is created by CI during release workflow
- For local testing, manually create structure or see `scripts/build.sh`

#### Getting Help

- Read `.github/copilot-instructions.md` for AI assistant context
- Check existing issues for solutions
- Ask questions in issue comments
- Join discussions on PRs
- Open a new issue if stuck

### Improving The Documentation

BrainFoamKit has multiple layers of documentation that all need maintenance:

#### API Documentation (Rust Docs)

Located in inline doc comments throughout `src/`:

```bash
# Generate and view docs
cargo doc --no-deps --open

# Check for missing docs
cargo doc --no-deps 2>&1 | grep warning
```

**What to document:**

- All public functions, structs, enums, traits
- Complex private functions that aren't obvious
- Module-level documentation (`//!` at top of file)

**Guidelines:**

- Include runnable examples with `# Examples`
- Link to related functions with backticks: `` [`other_function()`](#method.other_function) ``
- Document panics, errors, and edge cases
- Reference the Brainfuck language when relevant

#### User Guide (mdbook)

Located in `guide/src/`, built with mdbook:

```bash
cd guide

# Install mdbook if needed
cargo install mdbook mdbook-plantuml

# Build and serve locally
mdbook build
mdbook serve  # View at http://localhost:3000
```

**Sections:**

- `language_reference/` - BF language specification
- `language_implementation/` - Architecture docs
- `bfkrun/` - Interpreter usage
- `bfkview/` - TUI visualizer usage
- `getting_started.md` - Installation and quickstart

**When to update:**

- New features added to CLI/TUI
- Architecture changes in VM
- Breaking API changes
- New examples or tutorials

**Deployed to:** GitHub Pages at <https://alisajid.github.io/BrainFoamKit/>

#### Copilot Instructions

Located at `.github/copilot-instructions.md`:

**Purpose:** Guide AI coding assistants on:

- Project conventions and patterns
- Current MVP status and blockers
- Priority order for contributions
- Known architectural decisions

**Update when:**

- New patterns established
- MVP status changes
- Issue structure evolves
- Major refactoring happens

#### README and Top-Level Docs

**README.md:**

- Project overview and status
- Quick installation instructions
- Build badges
- Link to full documentation

**CHANGELOG.md:**

- Auto-generated from commit messages
- Don't edit manually - fix commit messages instead

**CONTRIBUTING.md:**

- This file! Keep it updated as workflows evolve

#### Language Specification

Located in `lang/`:

- `grammar.ebnf` - Formal EBNF grammar
- `README.md` - Language design notes
- `railroad/` - Railroad diagrams

**Update when:**

- Language semantics change
- Grammar evolves
- New instructions considered

## Style Guides

### Rust Code Style

BrainFoamKit enforces strict code quality through automated tooling.

#### Formatting with rustfmt

All code must be formatted with rustfmt:

```bash
cargo fmt --all
```

This is checked in CI - PRs with formatting issues will fail.

#### Linting with Clippy

We use **exceptionally strict** clippy lints configured in `Cargo.toml`:

- `-W clippy::pedantic` - Pedantic lints enabled
- `-W clippy::nursery` - Nursery (experimental) lints enabled
- `-W clippy::unwrap_used` - **No `unwrap()` allowed**
- `-W clippy::expect_used` - **No `expect()` allowed**

**Run clippy checks:**

```bash
bacon clippy  # Recommended: watch mode with auto-reload
# OR
cargo clippy --all-targets -- -D warnings
```

**Why so strict?**

- Prevents panics in production
- Forces proper error handling with `anyhow::Result`
- Catches common bugs early
- Maintains high code quality

**Suppressing Lints:**

- Avoid `#[allow(...)]` unless absolutely necessary
- Discuss with maintainers before suppressing
- Provide justification in code comments
- Only suppress at the narrowest scope possible

#### Architecture Patterns

See `.github/copilot-instructions.md` for detailed patterns:

**Builder Pattern:**

```rust
// ✅ CORRECT
let vm = VirtualMachine::builder()
    .tape_size(1024)
    .program(program)
    .input_device(stdin)
    .build()?;

// ❌ WRONG - Never construct directly
let vm = VirtualMachine::new(...);
```

**Error Handling:**

```rust
// ✅ CORRECT
fn increment_value(&mut self) -> Result<()> {
    let value = self.tape.get(self.pointer)
        .context("Pointer out of bounds")?;
    // ...
    Ok(())
}

// ❌ WRONG - No unwrap/expect
fn increment_value(&mut self) {
    let value = self.tape[self.pointer].unwrap();
    // ...
}
```

**Test Organization:**

```rust
// ✅ CORRECT - Tests inline with implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // test code
    }
}
```

#### Documentation Style

**Public APIs must have:**

- Description of what the function/struct does
- `# Arguments` section (if applicable)
- `# Returns` section
- `# Examples` section with working code
- `# Panics` section (if applicable)
- `# Errors` section (if returning `Result`)

**Example:**

```rust
/// Increments the value at the current memory pointer.
///
/// This wraps around from 255 to 0 (no overflow).
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::VirtualMachine;
///
/// let mut vm = VirtualMachine::builder().build()?;
/// vm.increment_value();
/// assert_eq!(vm.current_value(), 1);
/// ```
///
/// # See Also
///
/// * [`decrement_value()`](#method.decrement_value)
pub fn increment_value(&mut self) {
    // implementation
}
```

#### SPDX License Headers

**Every file** must include SPDX headers for licensing compliance:

**Rust source files:**

```rust
// SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
```

**Configuration files (TOML, YAML, etc.):**

```toml
# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: CC0-1.0
```

**Why this matters:**

- CI checks with `reuse lint` - PRs will fail without headers
- Ensures legal compliance
- Clarifies dual-licensing (MIT OR Apache-2.0)

#### File Organization

**Module structure:**

- Keep related code together
- Tests inline with `#[cfg(test)]`
- Re-export public APIs in `lib.rs`

**Imports:**

- Group standard library, external crates, local modules
- Use explicit imports (avoid `use crate::*`)
- Alphabetize within groups

```rust
// Standard library
use std::fmt::{Display, Formatter};
use std::io::Read;

// External crates
use anyhow::{Context, Result};

// Local modules
use crate::{Bit, Byte, Instruction};
```

## Automation Tools and Scripts

BrainFoamKit uses several automation tools to maintain quality and streamline development.

### Build Automation

#### bacon (Watch Mode)

Configured in `bacon.toml`, bacon provides continuous feedback during development:

```bash
# Default job: clippy with strict lints
bacon

# Available keyboard shortcuts:
# c - Run clippy
# ctrl-c - Check all targets
# t - Run tests
# a - Run cargo audit
# d - Generate docs
# o - Open docs in browser
```

**Jobs configured:**

- `clippy` - Default, runs on nightly with pedantic/nursery lints
- `check` / `check-all` - Basic compilation checks
- `test` - Run tests with colored output
- `test-nextest` - Run with nextest
- `doc` / `doc-open` - Generate and view documentation
- `audit` - Security audit with cargo-audit

**When to use:**

- Always! Run bacon in a terminal pane while coding
- Catches errors immediately as you type
- Faster than manual compilation

#### nextest (Test Runner)

Configured in `nextest.toml`, provides fast parallel test execution:

```bash
# Run all tests
cargo nextest run

# Run specific test
cargo nextest run test_name

# Show all output (including passed tests)
cargo nextest run --nocapture
```

**Configuration highlights:**

- `test-threads = "num-cpus"` - Parallel execution
- `status-level = "all"` - Shows all tests including skipped
- `failure-output = "immediate-final"` - Fast failure feedback
- `slow-timeout = 60s` - Warns on slow tests

### Release Automation

#### scripts/build.sh

Prepares release artifacts from CI builds:

```bash
# Run by CI, but can test locally
./scripts/build.sh
```

**What it does:**

1. Creates `artifacts/` directory structure
2. Extracts binary-target pairs from CI builds
3. Organizes for semantic-release packaging

**When to use:**

- Rarely needed locally (CI handles it)
- Debugging release workflow issues
- Testing artifact structure changes

#### scripts/generate_about_*.sh

Generate license compliance reports:

```bash
# Generate JSON report
./scripts/generate_about_json.sh

# Generate Markdown report
./scripts/generate_about_md.sh
```

**Outputs:**

- `licenses_report.json` - Machine-readable
- `licenses_report.md` - Human-readable

**Uses `about.toml` configuration** for license policies.

### Dependency Management

#### cargo-deny

Configured in `deny.toml`, enforces dependency policies:

```bash
# Check all policies
cargo deny check

# Check specific category
cargo deny check licenses
cargo deny check advisories
cargo deny check bans
cargo deny check sources
```

**Policies enforced:**

- **Licenses:** Only MIT, Apache-2.0, ISC allowed
- **Security:** No vulnerabilities above "Medium" CVSS
- **Bans:** Explicit deny list for problematic crates
- **Sources:** Only crates.io registry allowed

**Targets:** 10 platform triples configured

### Code Quality

#### Pre-commit Hooks

Installed with:

```bash
pip install pre-commit
pre-commit install
```

**Hooks run:**

- `rustfmt` - Format check
- `clippy` - Lint check
- `reuse lint` - SPDX header validation
- Trailing blank space removal
- YAML syntax validation

**Skip if needed (emergency only):**

```bash
git commit --no-verify
```

#### REUSE SPDX Compliance

Checks that all files have proper licensing headers:

```bash
# Check compliance
reuse lint

# Add headers to new files automatically
reuse addheader --copyright "Ali Sajid Imami" \
  --license MIT --license Apache-2.0 \
  src/new_file.rs
```

### CI/CD Workflows

Located in `.github/workflows/`:

**ci.yaml** - Main CI pipeline:

- Matrix: 3 OS × 4 Rust versions (stable/beta/nightly/MSRV 1.75.0)
- Runs: fmt, clippy, test, build, deny, reuse
- Path filters: Skips on docs-only changes

**release.yaml** - Semantic release:

- Triggered by push to `main` or `alpha` branches
- Runs `scripts/build.sh` for artifact preparation
- Uses Conventional Commits for version bumping
- Publishes to GitHub Releases

**deploy_mdbook.yaml** - Documentation:

- Deploys `guide/` to GitHub Pages
- Runs on push to `main` or manual trigger

**Code coverage** (future):

- Planned integration with codecov.io
- See [#423](https://github.com/AliSajid/BrainFoamKit/issues/423)

### Commit Messages

We use the [Conventional Commits](https://www.conventionalcommits.org/) specification for all commit messages. This leads to **more readable messages** that are easy to follow when looking through the **project history**. The commit messages are also used to **automatically generate the CHANGELOG** during releases.

#### Format

```text
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

#### Types

- `feat` - A new feature
- `fix` - A bug fix
- `docs` - Documentation changes only
- `style` - Code style changes (formatting, missing semicolons, etc.)
- `refactor` - Code changes that neither fix a bug nor add a feature
- `perf` - Performance improvements
- `test` - Adding or correcting tests
- `chore` - Changes to build process or auxiliary tools
- `ci` - Changes to CI configuration files and scripts

#### Scopes

Common scopes in BrainFoamKit:

- `vm` - Virtual machine core
- `cli` - Command-line interpreter (bfkrun)
- `tui` - Terminal UI visualizer (bfkview)
- `primitives` - Bit/Nybble/Byte types
- `parser` - Instruction parsing
- `ci` - CI/CD workflows
- `deps` - Dependency updates
- `docs` - Documentation (guide, README, etc.)

#### Examples

```bash
# Feature addition
git commit -m "feat(vm): implement jump_forward for loop control"

# Bug fix
git commit -m "fix(cli): handle missing input files gracefully"

# Documentation
git commit -m "docs(guide): add VM architecture diagram"

# Breaking change (adds ! or BREAKING CHANGE footer)
git commit -m "feat(vm)!: change VirtualMachine constructor to private

BREAKING CHANGE: VirtualMachine can only be constructed via builder pattern.
Use VirtualMachine::builder() instead of VirtualMachine::new()."

# Multiple paragraphs
git commit -m "refactor(primitives): optimize Byte increment operation

Previously used recursive bit flipping which was slow for increments
near byte boundaries. Now uses carry propagation with early exit.

Improves performance by ~40% for sequential increments."

# Closes issue
git commit -m "feat(vm): implement output_value instruction

Implements the '.' instruction to output current cell value.

Closes #415"
```

#### Guidelines

**DO:**

- Use present tense ("add feature" not "added feature")
- Use imperative mood ("move cursor" not "moves cursor")
- Start with lowercase (unless referencing proper names/types)
- Reference issues at the end of the body
- Keep subject line under 72 characters
- Add body for non-trivial changes

**DON'T:**

- End subject line with a period
- Use vague messages like "fix bug" or "update code"
- Forget to reference the issue being fixed
- Mix multiple unrelated changes in one commit

#### Commit Often

Break work into logical commits:

- One feature/fix per commit
- Each commit should build and pass tests
- Easier to review and bisect if issues arise

#### Amending Commits

Before pushing, you can amend the last commit:

```bash
# Fix something in the last commit
git add fixed_file.rs
git commit --amend --no-edit

# Change the last commit message
git commit --amend
```

**After pushing:** Don't amend! Create a new commit instead.

## Join The Project Team

We are always open to people joining our team. Please [open an issue](https://github.com/AliSajid/brainfoamkit/issues) to alert the team that you are interested in joining. We will then discuss the details in the issue.

<!-- omit in toc -->
