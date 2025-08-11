use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, init};

use chimera_twm::runtime::input::{TypeHandler,TypeHold,TypePress,TypeSpam,input_handler};

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


fn on_a_press() { println!("pressed A"); }
fn on_a_spam()  { println!("spamming A"); }
fn on_a_hold()  { println!("holding A"); }

// ---------------
// Rendering Area
// ---------------
fn alternative_screen_buffer(mut terminal: DefaultTerminal) -> Result<()> { 
    terminal.draw(render)?;
    loop {
        input_handler(TypeHandler {
            press: TypePress { key_code: KeyCode::Char('a'), handler: on_a_press },
            spam: Some(TypeSpam { handler: on_a_spam }),
            hold: Some(TypeHold { handler: on_a_hold }),
        });
    }
}


fn render(frame: &mut Frame) {
    frame.render_widget("hello", frame.area());
}
