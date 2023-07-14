use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppResult, Menu};

use super::handle_basic_keybindings;

pub fn handler(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.active_menu == Menu::Add {
    } else {
        handle_basic_keybindings(key_event, app)?;
        match key_event.code {
            KeyCode::Char('a') => app.active_menu = Menu::Add,
            KeyCode::Left => {}
            _ => {}
        }
    }
    Ok(())
}
