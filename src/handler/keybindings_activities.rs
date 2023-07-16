use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult, Menu},
    data::activity::Activity,
};

use super::handle_basic_keybindings;

pub fn handler(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_menu {
        Menu::Main => {
            handle_basic_keybindings(key_event, app)?;
            match key_event.code {
                KeyCode::Char('a') => app.active_menu = Menu::Add,
                KeyCode::Char('d') => app.active_menu = Menu::Delete,
                KeyCode::Down => app.select_activity(1),
                KeyCode::Up => app.select_activity(-1),
                _ => {}
            }
        }
        Menu::Add => {
            match app.activity_state.add.selected() {
                0 => match key_event.code {
                    KeyCode::Char(c) => app.activity_state.add.activity.name.push(c),
                    KeyCode::Backspace => {
                        app.activity_state.add.activity.name.pop();
                    }
                    _ => {}
                },
                1 => match key_event.code {
                    KeyCode::Right => app.activity_state.add.activity.color.next(),
                    KeyCode::Left => app.activity_state.add.activity.color.prev(),
                    _ => {}
                },
                2 => {
                    if key_event.code == KeyCode::Enter {
                        app.db
                            .new_activity(app.activity_state.add.activity.clone())
                            .unwrap();
                        app.activity_state.activities = app.db.get_activities();
                        app.active_menu = Menu::Main;
                        app.activity_state.add.activity = Activity::default();
                        app.activity_state.add.select_top();
                    }
                }
                _ => unimplemented!(),
            };
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => app.active_menu = Menu::Main,
                KeyCode::Down | KeyCode::Tab => app.activity_state.add.move_up(),
                KeyCode::Up => app.activity_state.add.move_down(),
                KeyCode::Right | KeyCode::Left => {}
                _ => {}
            }
        }
        Menu::Delete => match key_event.code {
            KeyCode::Char(c) => app.activity_state.delete_confirm.push(c),
            KeyCode::Backspace => {
                app.activity_state.delete_confirm.pop();
            }
            KeyCode::Esc => app.active_menu = Menu::Main,
            _ => {}
        },
        Menu::Edit => todo!(),
    }
    Ok(())
}
