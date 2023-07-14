mod keybindings_activities;
mod keybindings_exercises;
mod keybindings_overview;

use crate::{
    app::{App, AppResult},
    ui::tabs::Tab,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [App].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_tab {
        Tab::Overview => keybindings_overview::handler(key_event, app),
        Tab::Exercises => keybindings_exercises::handler(key_event, app),
        Tab::Workouts => handle_basic_keybindings(key_event, app),
        Tab::Activities => keybindings_activities::handler(key_event, app),
    }
    .unwrap();
    Ok(())
}

/// Handles the basic keybinding of the TUI:
/// * Esc & q for exiting the TUI
/// * ctrl+c for exiting the TUI
/// * Arrow right & left and Tab for navigating the tabs
fn handle_basic_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Right | KeyCode::Tab => app.active_tab.next(),
        KeyCode::Left => app.active_tab.prev(),
        _ => {}
    }
    Ok(())
}
