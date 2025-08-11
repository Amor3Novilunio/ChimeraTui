use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use ratatui;

// ---------------------
// Params
// key_code -> key bindings
// key_handler -> what happens if that key binding is pressed
// spam -> what would happen if spam is clicked
// hold -> what would happen if you press hold on key
// ---------------------
pub struct TypeHandler {
    pub press: TypePress,
    pub spam: Option<TypeSpam>,
    pub hold: Option<TypeHold>,
}

pub struct TypePress {
    pub key_code: KeyCode,
    pub handler: fn(),
}

pub struct TypeSpam {
    pub handler: fn(),
}

pub struct TypeHold {
    pub handler: fn(),
}

pub fn input_handler(handler: TypeHandler) {
    // ---------------------
    // Input Events Handler
    // ---------------------
    let events = match event::read() {
        Ok(res) => res,
        Err(err) => {
            ratatui::restore();
            eprintln!("Key : {} Error : {} ", handler.press.key_code, err);
            return;
        }
    };

    let spam_cooldown = Duration::from_millis(200);
    let hold_cooldown = Duration::from_millis(500);

    // -----------------------------------
    // Identify if the event is Key Input
    // -----------------------------------
    match events {
        Event::Key(key_event) => {
            // ---------------
            // Press Handling
            // ---------------
            if key_event.code == handler.press.key_code {
                let last_pressed_time = Instant::now();

                // --------------
                // Spam Handling | Spam press
                // check the spam if it exist in memory first
                // -------------------------------------------
                if let Some(_spam) = &handler.spam {
                    // ----------------------------------------------------------
                    // checks if the button is being pressed before its cooldown
                    // ----------------------------------------------------------
                    if last_pressed_time.elapsed() >= spam_cooldown {
                        (_spam.handler)();
                        return;
                    }
                    // proceed to next logic if the conditions are not met
                }

                // --------------
                // Hold Handling | Holding press
                // check the hold if it exist in memory first
                // -------------------------------------------
                if let Some(_hold) = &handler.hold {
                    // --------------------------------------
                    // checks if the button is being pressed
                    // --------------------------------------
                    if last_pressed_time.elapsed() >= hold_cooldown {
                        (_hold.handler)();
                        return;
                    }
                    // proceed to next logic if the conditions are not met
                }

                // Single Press
                (handler.press.handler)();
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
    };
}
