use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult, Menu},
    ui::log::LogArea,
};

use super::handle_basic_keybindings;

pub fn handler(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.active_menu == Menu::Add {
        if matches!(key_event.code, KeyCode::Enter | KeyCode::Tab) {
            if app.log_state.active_area == LogArea::Activity {
                app.log_state.active_area = LogArea::Exercise;
            } else {
                app.log_state.active_area = LogArea::Activity;
            }
        }
        if key_event.code == KeyCode::Char('q') {
            app.active_menu = Menu::Main;
        }

        match app.log_state.active_area {
            crate::ui::log::LogArea::Activity => {
                basic_activity_navigation(key_event, app);
                specific_activity_bindings(key_event, app);
            }
            crate::ui::log::LogArea::Exercise => match key_event.code {
                KeyCode::Right => app.log_state.horizontal_move_exercise_area(1),
                KeyCode::Left => app.log_state.horizontal_move_exercise_area(-1),
                KeyCode::Up => app.log_state.vertical_move_exercise_area(1),
                KeyCode::Down => app.log_state.vertical_move_exercise_area(-1),
                KeyCode::Char('a') => app
                    .log_state
                    .increase_selection(app.exercise_state.exercises.clone(), &app.db),
                KeyCode::Char('x') => app
                    .log_state
                    .decrease_selection(app.exercise_state.exercises.clone()),
                KeyCode::Char('s') => app.save_log(),
                KeyCode::Char('t') => app.log_state.timer.toggle(),
                KeyCode::Char('r') => app.log_state.timer.round(),
                _ => {}
            },
        }
    } else {
        handle_basic_keybindings(key_event, app)?;
        match key_event.code {
            KeyCode::Char('a') => app.active_menu = Menu::Add,
            KeyCode::Left => {}
            KeyCode::Down => {
                let new_selected = app.log_state.table.selected().unwrap() + 1;
                if new_selected < app.log_state.table_size {
                    app.log_state.table.select(Some(new_selected));
                }
            }
            KeyCode::Up => {
                let selected = app.log_state.table.selected().unwrap();
                if selected > 0 {
                    app.log_state.table.select(Some(selected - 1));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn basic_activity_navigation(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Down => app.log_state.offset_selected_activity_row(1),
        KeyCode::Up => app.log_state.offset_selected_activity_row(-1),
        _ => {}
    }
}

fn specific_activity_bindings(key_event: KeyEvent, app: &mut App) {
    match app.log_state.selected_activity_row {
        0 => match key_event.code {
            // Activity
            KeyCode::Right => app.select_log_activity(1),
            KeyCode::Left => app.select_log_activity(-1),
            _ => {}
        },
        1 => match key_event.code {
            // Date
            KeyCode::Right => app.log_state.selected_time.next(),
            KeyCode::Left => app.log_state.selected_time.prev(),
            KeyCode::Char('a') => app.log_state.increase_time(),
            KeyCode::Char('x') => app.log_state.decrease_time(),
            _ => {}
        },
        2 => match key_event.code {
            // Intensity
            KeyCode::Right | KeyCode::Char('a') => app.log_state.increase_intensity(),
            KeyCode::Left | KeyCode::Char('x') => app.log_state.decrease_intensity(),
            _ => {}
        },
        3 => match key_event.code {
            KeyCode::Char(c) => app.log_state.comment.push(c),
            KeyCode::Backspace => {
                app.log_state.comment.pop();
            }
            _ => {}
        },

        _ => {}
    }
}
