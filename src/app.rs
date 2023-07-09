use std::error;

use tui::widgets::TableState;

use crate::{
    data::{Activity, DB},
    ui::{tabs::Tab, AddActivityState},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub active_tab: Tab,
    pub active_area: ActiveArea,
    pub db: DB,
    pub activities: Vec<Activity>,
    pub activity_state: ActivityState,
}

impl Default for App {
    fn default() -> Self {
        let db = DB::default();
        let activities = db.get_activities();
        Self {
            running: true,
            active_tab: Tab::Activities,
            active_area: ActiveArea::default(),
            db,
            activities,
            activity_state: ActivityState {
                table: TableState::default().with_selected(Some(0)),
                ..Default::default()
            },
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

    pub fn remove_activity(&mut self) {
        let selected = self.activity_state.table.selected().unwrap();
        self.db.remove_activity(self.activities[selected].id);
        self.activities = self.db.get_activities();
        if selected > 0 {
            self.activity_state.table.select(Some(selected - 1));
        }
    }

    pub fn select_activity(&mut self, offset: isize) {
        let selected = self.activity_state.table.selected().unwrap();
        if offset > 0 {
            if selected < self.activities.len() - 1 {
                self.activity_state.table.select(Some(selected + 1));
            }
        } else if offset < 0 && selected > 0 {
            self.activity_state.table.select(Some(selected - 1));
        }
    }
}

#[derive(PartialEq)]
pub enum ActiveArea {
    Main,
    AddActivity,
}

impl Default for ActiveArea {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(Default)]
pub struct ActivityState {
    pub add: AddActivityState,
    pub table: TableState,
}
