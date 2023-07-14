pub mod add_activity_window;
mod delete_activity_window;

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
    data::{activity::Activity, DB},
};

use super::{basic_layout, render_tabs, AddActivityState};

pub struct ActivityState {
    pub activities: Vec<Activity>,
    pub add: AddActivityState,
    pub delete_confirm: String,
    pub table: TableState,
}

impl ActivityState {
    pub fn new(db: &DB) -> Self {
        let activities = db.get_activities();
        Self {
            activities,
            add: AddActivityState::default(),
            delete_confirm: String::default(),
            table: TableState::default().with_selected(Some(0)),
        }
    }
}

/// Renders the activities menu into the specified frame.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let layout = basic_layout(frame);

    render_tabs(frame, app, layout[0]);

    let mut activity_rows = Vec::with_capacity(app.activity_state.activities.len());

    for a in app.activity_state.activities.iter() {
        activity_rows.push(Row::new(vec![
            Span::from(format!("{}", a.id)),
            Span::from(a.name.clone()),
            Span::styled("■", Style::new().fg(a.color.into())),
            Span::from(a.symbol.clone()),
            Span::from(match a.has_exercise {
                true => "☒", // TODO: Nur Häkchen Symbol?
                false => "",
            }),
        ]))
    }

    // Render activity part
    frame.render_stateful_widget(
        ratatui::widgets::Table::new(activity_rows)
            .header(
                Row::new(vec!["ID", "Name", "Color", "Symbol", "Exercises"]).style(
                    Style::new()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            )
            .widths(&[
                Constraint::Length(3),
                Constraint::Length(18),
                Constraint::Length(5),
                Constraint::Length(6),
                Constraint::Length(14),
            ])
            .highlight_style(if !app.activity_state.activities.is_empty() {
                Style::default().fg(app.activity_state.activities
                    [app.activity_state.table.selected().unwrap_or(0)]
                .color
                .into())
            } else {
                Style::default()
            })
            .highlight_symbol(">> ")
            .block(
                Block::default()
                    .title("Activities")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        layout[1],
        &mut app.activity_state.table,
    );

    // TODO:
    // Render logged activities of the selected type
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
        Menu::Add => add_activity_window::draw(frame, app),
        Menu::Delete => delete_activity_window::draw(frame, app),
        _ => {}
    }
}
