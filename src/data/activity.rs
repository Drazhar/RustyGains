use super::color::Color;

#[derive(Debug, Default, Clone)]
pub struct Activity {
    pub id: u64,
    pub name: String,
    pub color: Color,
    pub symbol: String,
    pub has_exercise: bool,
}
