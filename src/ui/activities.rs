pub mod add_activity_window;

use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Row},
    Frame,
};

use crate::app::{ActiveArea, App};

use super::{basic_layout, render_tabs};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let layout = basic_layout(frame);

    render_tabs(frame, app, layout[0]);

    let mut table_rows = Vec::with_capacity(app.activities.len());

    for a in app.activities.iter() {
        table_rows.push(Row::new(vec![
            Span::from(format!("{}", a.id)),
            Span::from(a.name.clone()),
            Span::styled("■", Style::new().fg(a.color.into())),
            Span::from(a.symbol.clone()),
            Span::from(match a.has_exercise {
                true => "☒",
                false => "☐",
            }),
        ]))
    }

    frame.render_stateful_widget(
        tui::widgets::Table::new(table_rows)
            .header(
                Row::new(vec!["ID", "Name", "Color", "Symbol", "Exercises"]).style(
                    Style::new()
                        .fg(tui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            )
            .widths(&[
                Constraint::Length(4),
                Constraint::Length(18),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(14),
            ])
            .highlight_style(if !app.activities.is_empty() {
                Style::default().fg(app.activities
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
    frame.render_widget(
        tui::widgets::Paragraph::new("").block(
            Block::default()
                .title("Logs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        layout[2],
    );

    if app.active_area == ActiveArea::AddActivity {
        add_activity_window::draw(frame, app);
    }
}
