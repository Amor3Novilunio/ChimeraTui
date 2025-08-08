use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, init};

// -----------
// Entry Point
// -----------
fn main() -> Result<()> {
    // --------------------------------------
    // Error Handler ( adds colorful errors )
    // --------------------------------------
    color_eyre::install()?;
    // ----------------------------------------------------------------
    // Run the terminal in raw mode ( [ buffer auto flush ] aka echo )
    // ----------------------------------------------------------------
    let terminal = init();
    // ------------------------------------------------------------------------
    // initialize's the alternative screen buffer of ratatui for the terminal
    // ------------------------------------------------------------------------
    let result = alternative_screen_buffer(terminal);
    // --------------------------------------------------------------------------------------
    // exits the alternative screen buffer of ratatui and restores the terminal where it ran
    // --------------------------------------------------------------------------------------
    ratatui::restore();
    // -----------------------------------------------------------------------------------------------------------------
    // returns the error fo the result to main to display it in the terminal after exiting the alternative screen buffer
    // -----------------------------------------------------------------------------------------------------------------
    result
}

// ---------------
// Rendering Area
// ---------------
fn alternative_screen_buffer(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}


fn render(frame: &mut Frame) {
    frame.render_widget("hello", frame.area());
}
