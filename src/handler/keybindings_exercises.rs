use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult, Menu},
    data::exercise::Exercise,
};

use super::handle_basic_keybindings;

pub fn handler(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_menu {
        Menu::Main => {
            handle_basic_keybindings(key_event, app)?;
            match key_event.code {
                KeyCode::Char('a') => app.active_menu = Menu::Add,
                KeyCode::Char('d') => app.active_menu = Menu::Delete,
                KeyCode::Down => app.select_exercise(1),
                KeyCode::Up => app.select_exercise(-1),
                _ => {}
            }
        }
        Menu::Add => {
            match app.exercise_state.add.selected() {
                0 => match key_event.code {
                    KeyCode::Char(c) => app.exercise_state.add.exercise.name.push(c),
                    KeyCode::Backspace => {
                        app.exercise_state.add.exercise.name.pop();
                    }
                    _ => {}
                },
                1 => match key_event.code {
                    KeyCode::Right => app.exercise_state.add.exercise.color.next(),
                    KeyCode::Left => app.exercise_state.add.exercise.color.prev(),
                    _ => {}
                },
                2 => match key_event.code {
                    KeyCode::Char(c) => app.exercise_state.add.exercise.description.push(c),
                    KeyCode::Backspace => {
                        app.exercise_state.add.exercise.description.pop();
                    }
                    _ => {}
                },
                3 => {
                    if key_event.code == KeyCode::Enter {
                        app.db
                            .new_exercise(app.exercise_state.add.exercise.clone())
                            .unwrap();
                        app.exercise_state.exercises = app.db.get_exercises();
                        app.active_menu = Menu::Main;
                        app.exercise_state.add.exercise = Exercise::default();
                        app.exercise_state.add.select_top();
                    }
                }
                _ => {
                    unimplemented!();
                }
            };
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => app.active_menu = Menu::Main,
                KeyCode::Down | KeyCode::Tab => app.exercise_state.add.move_up(),
                KeyCode::Up => app.exercise_state.add.move_down(),
                KeyCode::Right | KeyCode::Left => {}
                _ => {}
            }
        }
        Menu::Delete => match key_event.code {
            KeyCode::Char(c) => app.exercise_state.delete_confirm.push(c),
            KeyCode::Backspace => {
                app.exercise_state.delete_confirm.pop();
            }
            KeyCode::Esc => app.active_menu = Menu::Main,
            _ => {}
        },
        Menu::Edit => todo!(),
    }
    Ok(())
}
