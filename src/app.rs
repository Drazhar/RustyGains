use std::error;

use crate::{
    data::DB,
    ui::{exercises::ExerciseState, ActivityState},
    ui::{log::LogState, tabs::Tab},
};

/// Quality of life result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// The state of the application.
pub struct App {
    pub running: bool,
    pub active_tab: Tab,
    pub active_menu: Menu,
    pub db: DB,
    pub activity_state: ActivityState,
    pub exercise_state: ExerciseState,
    pub log_state: LogState,
}

impl Default for App {
    fn default() -> Self {
        let db = DB::default();
        Self {
            running: true,
            active_tab: Tab::default(),
            active_menu: Menu::default(),
            activity_state: ActivityState::new(&db),
            exercise_state: ExerciseState::new(&db),
            log_state: LogState::new(&db),
            db,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn delete_activity(&mut self) {
        let table = &mut self.activity_state.table;
        let selected = table.selected().unwrap();

        self.db
            .delete_activity(self.activity_state.activities[selected].id);
        self.activity_state.activities = self.db.get_activities();
        if selected > 0 {
            table.select(Some(selected - 1));
        }
    }

    pub fn delete_exercise(&mut self) {
        let table = &mut self.exercise_state.table;
        let selected = table.selected().unwrap();

        self.db
            .delete_exercise(self.exercise_state.exercises[selected].id);
        self.exercise_state.exercises = self.db.get_exercises();
        if selected > 0 {
            table.select(Some(selected - 1));
        }
    }

    pub fn select_activity(&mut self, offset: isize) {
        let table = &mut self.activity_state.table;
        let selected = table.selected().unwrap();

        if offset > 0 {
            if selected < self.activity_state.activities.len() - 1 {
                table.select(Some(selected + 1));
            }
        } else if offset < 0 && selected > 0 {
            table.select(Some(selected - 1));
        }
    }

    pub fn select_exercise(&mut self, offset: isize) {
        let table = &mut self.exercise_state.table;
        let selected = table.selected().unwrap();

        if offset > 0 {
            if selected < self.exercise_state.exercises.len() - 1 {
                table.select(Some(selected + 1));
            }
        } else if offset < 0 && selected > 0 {
            table.select(Some(selected - 1));
        }
    }
}

/// Describes which sub menu is currently active
#[derive(PartialEq)]
pub enum Menu {
    Main,
    Add,
    Delete,
    Edit,
}

impl Default for Menu {
    fn default() -> Self {
        Self::Add
    }
}
