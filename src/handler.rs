use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.prev();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Char('r') => {
            app.set_status("Refreshing selected city…");
            // Actual fetch happens in main loop after this handler returns.
        }
        _ => {}
    }
    Ok(())
}