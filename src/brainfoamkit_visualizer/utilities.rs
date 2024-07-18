// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
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
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

/// Setup the terminal. This is where you would enable raw mode, enter the
/// alternate screen, and hide the cursor. This example does not handle errors.
/// A more robust application would probably want to handle errors and ensure
/// that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the
/// alternate screen, and show the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Run the application loop. This is where you would handle events and update
/// the application state. This example exits when the user presses 'q'. Other
/// styles of application loops are possible, for example, you could have
/// multiple application states and switch between them based on events, or you
/// could have a single application state and update it based on events.
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(render_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application. This is where you would draw the application UI.
/// This example just draws a greeting.
pub fn render_app(frame: &mut ratatui::Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}

/// Check if the user has pressed 'q'. This is where you would handle events.
/// This example just checks if the user has pressed 'q' and returns true if
/// they have. It does not handle any other events. There is a 250ms timeout on
/// the event poll so that the application can exit in a timely manner, and to
/// ensure that the terminal is rendered at least once every 250ms.
pub fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
