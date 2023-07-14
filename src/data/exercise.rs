use super::color::Color;

#[derive(Debug, Default, Clone)]
pub struct Exercise {
    pub id: u64,
    pub name: String,
    pub color: Color,
    pub description: String,
}

impl Exercise {
    pub fn description_tail(&self, n: usize) -> String {
        if n > self.description.len() {
            self.description.clone()
        } else {
            let start = self
                .description
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.description.len() - n)
                .unwrap();
            self.description[start..].to_owned()
        }
    }
}
