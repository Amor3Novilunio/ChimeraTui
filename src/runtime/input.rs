use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui;

// ---------------------
// Params
// key_code -> key bindings
// key_handler -> what happens if that key binding is pressed
// spam -> what would happen if spam is clicked
// hold -> what would happen if you press hold on key
// ---------------------
pub struct InputProps {
    pub key_code: KeyCode,
    pub key_modifiers: KeyModifiers,
    pub handler: fn(),
}

pub fn input_handler(input: InputProps) {
    // ---------------------
    // Input Events Handler
    // ---------------------
    let events = match event::read() {
        Ok(res) => res,
        Err(err) => {
            ratatui::restore();
            eprintln!("Key : {} Error : {} ", input.key_code, err);
            return;
        }
    };

    // -----------------------------------
    // Identify if the event is Key Input
    // -----------------------------------
    match events {
        Event::Key(key_event) => {
            // ---------------
            // Press Handling
            // ---------------
            if key_event.code.to_string().to_lowercase()
                == input.key_code.to_string().to_lowercase()
                && key_event.modifiers == input.key_modifiers
            {
                if key_event.is_press() {
                    (input.handler)()
                }
            }

            // -------------
            // Exit Command
            // -------------
            if key_event.code == KeyCode::Char('q') {
                ratatui::restore();
                return;
            }
        }
        _ => {
            ratatui::restore();
            return;
        }
    }
}
