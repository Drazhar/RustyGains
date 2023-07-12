use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{ActiveMenu, App, AppResult},
    data::Activity,
};

use super::handle_basic_keybindings;

pub fn handler(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.active_area {
        ActiveMenu::Main => {
            handle_basic_keybindings(key_event, app)?;
            match key_event.code {
                KeyCode::Char('a') => app.active_area = ActiveMenu::AddActivity,
                KeyCode::Char('d') => app.active_area = ActiveMenu::DeleteActivity,
                KeyCode::Down => app.select_activity(1),
                KeyCode::Up => app.select_activity(-1),
                _ => {}
            }
        }
        ActiveMenu::AddActivity => {
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
                        app.active_area = ActiveMenu::Main;
                        app.activity_state.add.activity = Activity::default();
                        app.activity_state.add.select_top();
                    }
                }
                _ => unimplemented!(),
            };
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => app.active_area = ActiveMenu::Main,
                KeyCode::Down | KeyCode::Tab => app.activity_state.add.move_up(),
                KeyCode::Up => app.activity_state.add.move_down(),
                KeyCode::Right | KeyCode::Left => {}
                _ => {}
            }
        }
        ActiveMenu::DeleteActivity => match key_event.code {
            KeyCode::Char(c) => app.activity_state.delete_confirm.push(c),
            KeyCode::Backspace => {
                app.activity_state.delete_confirm.pop();
            }
            KeyCode::Esc => app.active_area = ActiveMenu::Main,
            _ => {}
        },
    }
    Ok(())
}
