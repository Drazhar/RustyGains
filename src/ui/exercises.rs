pub mod add_exercise_window;
mod delete_exercise_window;

use ratatui::{
    backend::Backend,
    layout::Constraint,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Row, TableState},
    Frame,
};

use crate::{
    app::{App, Menu},
    data::{exercise::Exercise, DB},
};

use self::add_exercise_window::AddExerciseState;

use super::{basic_layout, render_tabs};

pub struct ExerciseState {
    pub add: AddExerciseState,
    pub delete_confirm: String,
    pub table: TableState,
    pub exercises: Vec<Exercise>,
}

impl ExerciseState {
    pub fn new(db: &DB) -> Self {
        let exercises = db.get_exercises();
        Self {
            exercises,
            add: AddExerciseState::default(),
            delete_confirm: String::default(),
            table: TableState::default().with_selected(Some(0)),
        }
    }
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let layout = basic_layout(frame);

    render_tabs(frame, app, layout[0]);

    let mut exercise_rows = Vec::with_capacity(app.exercise_state.exercises.len());

    for e in app.exercise_state.exercises.iter() {
        exercise_rows.push(Row::new(vec![
            Span::from(format!("{}", e.id)),
            Span::from(e.name.clone()),
            Span::styled("■", Style::new().fg(e.color.into())),
            Span::from(e.description.clone()),
        ]))
    }

    // Render activity part
    frame.render_stateful_widget(
        ratatui::widgets::Table::new(exercise_rows)
            .header(
                Row::new(vec!["ID", "Name", "Color", "Description"]).style(
                    Style::new()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            )
            .widths(&[
                Constraint::Length(3),
                Constraint::Length(18),
                Constraint::Length(5),
                Constraint::Percentage(100),
            ])
            .highlight_style(if !app.exercise_state.exercises.is_empty() {
                Style::default().fg(app.exercise_state.exercises
                    [app.exercise_state.table.selected().unwrap_or(0)]
                .color
                .into())
            } else {
                Style::default()
            })
            .highlight_symbol(">> ")
            .block(
                Block::default()
                    .title("Exercises")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        layout[1],
        &mut app.exercise_state.table,
    );

    // TODO: Show logs from that exercise
    frame.render_widget(
        ratatui::widgets::Paragraph::new("").block(
            Block::default()
                .title("Logs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        layout[2],
    );

    match app.active_menu {
        Menu::Add => add_exercise_window::draw(frame, app),
        Menu::Delete => delete_exercise_window::draw(frame, app),
        _ => {}
    }
}
