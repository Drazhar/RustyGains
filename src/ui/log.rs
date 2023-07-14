use crate::data::{ActivityLog, DB};

pub struct LogState {
    pub logs: Vec<ActivityLog>,
    pub active_area: LogArea,
}

impl LogState {
    pub fn new(db: &DB) -> Self {
        Self {
            logs: db.get_logs(),
            active_area: LogArea::Activity,
        }
    }
}

pub enum LogArea {
    Activity,
    Exercise,
}
