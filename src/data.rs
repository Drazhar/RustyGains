pub mod activity;
pub mod color;
pub mod exercise;
mod schema;


use chrono::NaiveDateTime;
use rusqlite::Connection;

use crate::{app::{AppResult, App}, ui::log::ExerciseLog};

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

    pub fn get_last_exercise_log(&self) -> AppResult<ExerciseLog> {
        Ok(self.con
            .query_row("SELECT exercise_id, exercise.name, exercise.color, exercise.description, weight, break, reps, effort FROM exercise_log JOIN exercise ON exercise_id = exercise.id;", [], |row| {
                let splitted = text_to_vec(row.get(6)?);
                
                Ok(ExerciseLog {
                    exercise: Exercise {
                         id: row.get(0)?,
                         name: row.get(1)?,
                         color: row.get(2)?,
                         description: row.get(3)?
                    },
                    weight: row.get(4)?,
                    breaks: row.get(5)?,
                    reps: splitted,
                    effort: row.get(7)?,
                })
            })
            ?)
    }

    pub fn get_exercise_history(&self, id: u64) -> Vec<ExerciseHistory> {
        let mut stmt = self.con.prepare("
                SELECT
                  activity_log.date, weight, break, reps, effort, comment
                FROM
                  exercise_log
                JOIN
                  activity_log
                ON
                  activity_log.id = activity_log_id
                WHERE
                  exercise_id = ?1
                ORDER BY
                  date DESC
            ").unwrap();

        stmt.query_map([id], |row| {
        let reps = text_to_vec(row.get(3)?);
        let timestamp: i64 = row.get(0)?;
            Ok(ExerciseHistory{ 
                date: NaiveDateTime::from_timestamp_millis(timestamp * 1000).unwrap(),
                weight: row.get(1)?,
                breaks: row.get(2)?,
                reps,
                effort: row.get(4)?,
                comment: row.get(5)?
            })}).unwrap().map(|e| e.unwrap()).collect()
    }
    
    pub fn get_activity_log(&self, activity_name: &str) -> Vec<ActivityLog> {
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
                WHERE
                  activity.name = ?1
                ORDER BY
                  date DESC
                ;",
            )
            .unwrap();

        let activity_logs: Vec<ActivityLog> = stmt
            .query_map([activity_name], |row| {
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

    pub fn save_log(&self, log: &App) {
    self.con.execute(
            "INSERT INTO activity_log (activity_id, date, intensity, comment) VALUES (?1, ?2, ?3, ?4)",
            (log.activity_state.activities[log.log_state.selected_activity].id, log.log_state.date.timestamp(), log.log_state.intensity, log.log_state.comment.clone())).unwrap();
        
    }
}

fn text_to_vec<T>(inp: String) -> Vec<T>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    let splitted: Vec<T> = inp.split(',').map(|e| e.parse().unwrap()).collect();
    splitted
}

pub struct ActivityLog {
    pub id: usize,
    pub activity: Activity,
    pub date: u64,
    pub intensity: u8,
    pub comment: String,
}

pub struct ExerciseHistory {
    pub date: NaiveDateTime,
    pub weight: f64,
    pub breaks: f64,
    pub reps: Vec<u8>,
    pub effort: u8,
    pub comment: String
}
