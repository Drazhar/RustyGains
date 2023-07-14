pub mod activity;
pub mod color;
pub mod exercise;
mod schema;

use rusqlite::Connection;

use crate::app::AppResult;

use self::{activity::Activity, exercise::Exercise};

pub struct DB {
    con: Connection,
}

impl Default for DB {
    fn default() -> Self {
        let con = Connection::open("test.db").unwrap();
        schema::initialize(&con);
        Self { con }
    }
}

impl DB {
    pub fn get_activities(&self) -> Vec<Activity> {
        let mut stmt = self.con.prepare("SELECT * FROM activity;").unwrap();

        let activities: Vec<Activity> = stmt
            .query_map([], |row| {
                Ok(Activity {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    color: row.get(2).unwrap(),
                    symbol: row.get(3).unwrap(),
                    has_exercise: row.get(4).unwrap_or(false),
                })
            })
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        activities
    }

    pub fn get_exercises(&self) -> Vec<Exercise> {
        let mut stmt = self.con.prepare("SELECT * FROM exercise;").unwrap();

        let exercises: Vec<Exercise> = stmt
            .query_map([], |row| {
                Ok(Exercise {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    color: row.get(2).unwrap(),
                    description: row.get(3).unwrap(),
                })
            })
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        exercises
    }

    pub fn get_logs(&self) -> Vec<ActivityLog> {
        let mut stmt = self
            .con
            .prepare(
                "
                SELECT
                  activity_log.id, date, intensity, comment, activity.id,
                  activity.name, activity.color, activity.symbol,
                  activity.has_exercises  
                FROM 
                  activity_log
                JOIN 
                  activity
                ON
                  activity.id = activity_id
                ;",
            )
            .unwrap();

        let activity_logs: Vec<ActivityLog> = stmt
            .query_map([], |row| {
                Ok(ActivityLog {
                    id: row.get(0).unwrap(),
                    date: row.get(1).unwrap(),
                    intensity: row.get(2).unwrap(),
                    comment: row.get(3).unwrap(),
                    activity: Activity {
                        id: row.get(4).unwrap(),
                        name: row.get(5).unwrap(),
                        color: row.get(6).unwrap(),
                        symbol: row.get(7).unwrap(),
                        has_exercise: row.get(8).unwrap_or(false),
                    },
                })
            })
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        activity_logs
    }

    pub fn new_activity(&self, activity: Activity) -> AppResult<()> {
        self.con.execute(
            "INSERT INTO activity (name, color, symbol, has_exercises) VALUES (?1, ?2, ?3, ?4)",
            (
                activity.name,
                activity.color,
                activity.symbol,
                activity.has_exercise,
            ),
        )?;
        Ok(())
    }

    pub fn new_exercise(&self, exercise: Exercise) -> AppResult<()> {
        self.con.execute(
            "INSERT INTO exercise (name, color, description) VALUES (?1, ?2, ?3)",
            (exercise.name, exercise.color, exercise.description),
        )?;
        Ok(())
    }

    pub fn delete_activity(&self, id: u64) {
        self.con
            .execute("DELETE FROM activity WHERE id=?1", [id])
            .unwrap();
    }

    pub fn delete_exercise(&self, id: u64) {
        self.con
            .execute("DELETE FROM exercise WHERE id=?1", [id])
            .unwrap();
    }
}

pub struct ActivityLog {
    id: usize,
    activity: Activity,
    date: u64,
    intensity: u8,
    comment: String,
}

pub struct Log {}
