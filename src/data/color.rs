use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl FromSql for Color {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(c) => match c[0] {
                b'b' => Ok(Self::Blue),
                b'g' => Ok(Self::Green),
                b'y' => Ok(Self::Yellow),
                b'w' => Ok(Self::White),
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
            Color::White => Ok(ToSqlOutput::from("w")),
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
            Color::White => "White",
        }
    }
}

impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => ratatui::style::Color::Red,
            Color::Blue => ratatui::style::Color::Blue,
            Color::Green => ratatui::style::Color::Green,
            Color::Yellow => ratatui::style::Color::Yellow,
            Color::White => ratatui::style::Color::White,
        }
    }
}

impl Color {
    pub fn next(&mut self) {
        match self {
            Self::Red => *self = Self::Blue,
            Self::Blue => *self = Self::Green,
            Self::Green => *self = Self::Yellow,
            Self::Yellow => *self = Self::White,
            Self::White => *self = Self::Red,
        }
    }

    pub fn prev(&mut self) {
        match self {
            Self::Red => *self = Self::White,
            Self::Blue => *self = Self::Red,
            Self::Green => *self = Self::Blue,
            Self::Yellow => *self = Self::Green,
            Self::White => *self = Self::Yellow,
        }
    }
}
