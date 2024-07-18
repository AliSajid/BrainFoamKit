// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod utilities;
use anyhow::{
    Context,
    Result,
};

/// This is a bare minimum example. There are many approaches to running an
/// application loop, so this is not meant to be prescriptive. It is only meant
/// to demonstrate the basic setup and teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure
/// that the terminal is restored to a sane state before exiting. This example
/// does not do that. It also does not handle events or update the application
/// state. It just draws a greeting and exits when the user presses 'q'.
fn main() -> Result<()> {
    let mut terminal = utilities::setup_terminal().context("setup failed")?;
    utilities::run(&mut terminal).context("app loop failed")?;
    utilities::restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
