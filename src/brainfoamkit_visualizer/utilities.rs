// SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    io::{
        self,
        Stdout,
    },
    time::Duration,
};

use anyhow::{
    Context,
    Result,
};
use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

/// Setup the terminal for TUI mode: enable raw mode (disable line buffering),
/// enter alternate screen buffer (preserves the previous terminal state), and
/// hide the cursor. Returns a ratatui Terminal for drawing operations.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal to normal mode: disable raw mode (re-enable line
/// buffering), leave alternate screen buffer (returns to the previous terminal
/// state), and show the cursor again.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Run the application event loop: render the UI and check for user input in a
/// loop. The event polling timeout (250ms) ensures responsive UI updates even
/// without user input, while keeping CPU usage reasonable by not spinning
/// continuously.
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(render_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application UI. This is the draw callback invoked once per
/// frame by the terminal. Currently displays a simple greeting message.
pub fn render_app(frame: &mut ratatui::Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

/// Check if user pressed 'q' to quit. Uses a 250ms timeout on event polling:
/// if no input within 250ms, returns false so the UI can render. If 'q' is
/// pressed, returns true to signal application exit. Other key events are
/// silently ignored.
pub fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
