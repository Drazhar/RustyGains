mod schema;

use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};

use crate::app::AppResult;

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

    pub fn new_activity(&self, activity: Activity) -> AppResult<Activity> {
        self.con.execute(
            "INSERT INTO activity (name, color, symbol, has_exercises) VALUES (?1, ?2, ?3, ?4)",
            (
                activity.name,
                activity.color,
                activity.symbol,
                activity.has_exercise,
            ),
        )?;
        let inserted_id = self.con.last_insert_rowid();
        let result =
            self.con
                .query_row("SELECT * FROM activity WHERE id=?1", [inserted_id], |row| {
                    Ok(Activity {
                        id: row.get(0).unwrap(),
                        name: row.get(1).unwrap(),
                        color: row.get(2).unwrap(),
                        symbol: row.get(3).unwrap(),
                        has_exercise: row.get(4).unwrap_or(false),
                    })
                })?;
        Ok(result)
    }

    pub fn remove_activity(&self, id: u64) {
        self.con
            .execute("DELETE FROM activity WHERE id=?1", [id])
            .unwrap();
    }
}

#[derive(Debug, Default, Clone)]
pub struct Activity {
    pub id: u64,
    pub name: String,
    pub color: Color,
    pub symbol: String,
    pub has_exercise: bool,
}

impl Activity {
    pub fn next_color(&mut self) {
        match self.color {
            Color::Red => self.color = Color::Blue,
            Color::Blue => self.color = Color::Green,
            Color::Green => self.color = Color::Yellow,
            Color::Yellow => self.color = Color::Red,
        }
    }
    pub fn prev_color(&mut self) {
        match self.color {
            Color::Red => self.color = Color::Yellow,
            Color::Blue => self.color = Color::Red,
            Color::Green => self.color = Color::Blue,
            Color::Yellow => self.color = Color::Green,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Default for Color {
    fn default() -> Self {
        Self::Red
    }
}

impl FromSql for Color {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(c) => match c[0] {
                b'b' => Ok(Self::Blue),
                b'g' => Ok(Self::Green),
                b'y' => Ok(Self::Yellow),
                _ => Ok(Self::Red),
            },
            _ => Ok(Self::Red),
        }
    }
}

impl ToSql for Color {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Color::Red => Ok(ToSqlOutput::from("r")),
            Color::Blue => Ok(ToSqlOutput::from("b")),
            Color::Green => Ok(ToSqlOutput::from("g")),
            Color::Yellow => Ok(ToSqlOutput::from("y")),
        }
    }
}

impl From<Color> for &str {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Yellow => "Yellow",
        }
    }
}

impl From<Color> for tui::style::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => tui::style::Color::Red,
            Color::Blue => tui::style::Color::Blue,
            Color::Green => tui::style::Color::Green,
            Color::Yellow => tui::style::Color::Yellow,
        }
    }
}
