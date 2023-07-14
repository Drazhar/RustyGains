use ratatui::text::Line;

// WARN: Every enum has to be added to the "line_vec" function!
pub enum Tab {
    Overview,
    Exercises,
    Workouts,
    Activities,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Overview
    }
}

impl Tab {
    pub fn line_vec() -> Vec<Line<'static>> {
        ["Overview", "Exercises", "Workouts", "Activities"]
            .iter()
            .cloned()
            .map(ratatui::text::Line::from)
            .collect()
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Tab::Overview => 0,
            Tab::Exercises => 1,
            Tab::Workouts => 2,
            Tab::Activities => 3,
        }
    }

    pub fn next(&mut self) {
        match self {
            Tab::Overview => *self = Tab::Exercises,
            Tab::Exercises => *self = Tab::Workouts,
            Tab::Workouts => *self = Tab::Activities,
            Tab::Activities => *self = Tab::Overview,
        }
    }

    pub fn prev(&mut self) {
        match self {
            Tab::Overview => *self = Tab::Activities,
            Tab::Exercises => *self = Tab::Overview,
            Tab::Workouts => *self = Tab::Exercises,
            Tab::Activities => *self = Tab::Workouts,
        }
    }
}
