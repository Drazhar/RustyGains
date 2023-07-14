use ratatui::{
    backend::Backend,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, data::exercise::Exercise, settings, ui::floating_window};

const ADD_EXERCISE_ROWS: usize = 4;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let width = 60;
    let height = ADD_EXERCISE_ROWS as u16 + 2;
    let overlay = floating_window::create(frame, width, height);

    let state = &app.exercise_state.add;
    let exercise = &state.exercise;

    let mut options = vec![
        Line::from(vec![
            Span::from("Name         "),
            Span::from(String::from(&exercise.name)),
        ]),
        Line::from(vec![
            Span::from("Color        "),
            Span::styled(
                "■ ".to_owned() + exercise.color.into(),
                Style::default().fg(exercise.color.into()),
            ),
        ]),
        Line::from(vec![
            Span::from("Description  "),
            Span::from(exercise.description_tail(width as usize - 15)),
        ]),
        Line::from("Save"),
    ];

    if !options.is_empty() {
        options[state.selected].spans[0].style = Style::default().fg(settings::HIGHLIGHT_COLOR);
    }

    frame.render_widget(
        ratatui::widgets::Paragraph::new(options).block(
            Block::default()
                .title("Add new activity")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(settings::HIGHLIGHT_COLOR)),
        ),
        overlay,
    );
}

#[derive(Default)]
pub struct AddExerciseState {
    selected: usize,
    pub exercise: Exercise,
}

impl AddExerciseState {
    pub fn move_up(&mut self) {
        if self.selected == ADD_EXERCISE_ROWS - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.selected == 0 {
            self.selected = ADD_EXERCISE_ROWS - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub(crate) fn select_top(&mut self) {
        self.selected = 0;
    }
}
