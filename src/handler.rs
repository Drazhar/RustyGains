use std::unimplemented;

use crate::{
    app::{ActiveArea, App, AppResult},
    data::Activity,
    ui::tabs::Tab,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_tab {
        Tab::Overview => handle_overview_keybindings(key_event, app),
        Tab::Exercises => handle_exercises_keybindings(key_event, app),
        Tab::Workouts => handle_workouts_keybindings(key_event, app),
        Tab::Activities => handle_activities_keybindings(key_event, app),
    }
    .unwrap();
    Ok(())
}

fn handle_basic_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Navigate tabs
        KeyCode::Right | KeyCode::Tab => app.active_tab.next(),
        KeyCode::Left => app.active_tab.prev(),
        _ => {}
    }
    Ok(())
}

fn handle_overview_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    handle_basic_keybindings(key_event, app)?;
    match key_event.code {
        KeyCode::Char('a') => println!("Adding activity"),
        KeyCode::Down => println!("runter"),
        _ => {}
    }
    Ok(())
}
fn handle_exercises_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    handle_basic_keybindings(key_event, app)?;
    match key_event.code {
        KeyCode::Down => println!("runter"),
        KeyCode::Up => println!("hoch"),
        _ => {}
    }
    Ok(())
}
fn handle_workouts_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    handle_basic_keybindings(key_event, app)?;
    match key_event.code {
        KeyCode::Down => println!("runter"),
        KeyCode::Up => println!("hoch"),
        _ => {}
    }
    Ok(())
}
fn handle_activities_keybindings(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_area {
        ActiveArea::Main => {
            handle_basic_keybindings(key_event, app)?;
            match key_event.code {
                KeyCode::Char('a') => app.active_area = ActiveArea::AddActivity,
                KeyCode::Char('d') => app.remove_activity(),
                KeyCode::Down => app.select_activity(1),
                KeyCode::Up => app.select_activity(-1),
                _ => {}
            }
        }
        ActiveArea::AddActivity => {
            match app.activity_state.add.selected() {
                0 => match key_event.code {
                    KeyCode::Char(c) => app.activity_state.add.activity.name.push(c),
                    KeyCode::Backspace => {
                        app.activity_state.add.activity.name.pop();
                    }
                    _ => {}
                },
                1 => match key_event.code {
                    KeyCode::Right => app.activity_state.add.activity.next_color(),
                    KeyCode::Left => app.activity_state.add.activity.prev_color(),
                    _ => {}
                },
                2 => match key_event.code {
                    KeyCode::Right | KeyCode::Left | KeyCode::Enter => {
                        app.activity_state.add.activity.has_exercise =
                            !app.activity_state.add.activity.has_exercise;
                    }
                    _ => {}
                },
                3 => {
                    if key_event.code == KeyCode::Enter {
                        let acti = app.db.new_activity(app.activity_state.add.activity.clone());
                        app.activities.push(acti.unwrap());
                        app.active_area = ActiveArea::Main;
                        app.activity_state.add.activity = Activity::default();
                        app.activity_state.add.select_top();
                    }
                }
                _ => unimplemented!(),
            };
            match key_event.code {
                KeyCode::Char('q') => app.active_area = ActiveArea::Main,
                KeyCode::Down => app.activity_state.add.move_up(),
                KeyCode::Up => app.activity_state.add.move_down(),
                KeyCode::Right | KeyCode::Left => {}
                _ => {}
            }
        }
    }
    Ok(())
}
