use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Row},
    Frame,
};

use crate::{
    app::{ActiveArea, App},
    data::Activity,
    settings::{self, HIGHLIGHT_COLOR},
};

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
            .highlight_style(
                Style::new().fg(
                    app.activities[app.activity_state.table.selected().unwrap_or(0)]
                        .color
                        .into(),
                ),
            )
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
        let mut width = 60;
        let mut height = 6;
        let mut x = 0;
        let mut y = 0;

        if width > frame.size().width {
            width = frame.size().width;
        } else {
            x = (frame.size().width - width) / 2;
        }
        if height > frame.size().height {
            height = frame.size().height;
        } else {
            y = (frame.size().height - height) / 2;
        }

        let overlay = Rect::new(x, y, width, height);
        frame.render_widget(
            tui::widgets::Paragraph::new(vec![
                Line::from(str::repeat(
                    " ",
                    width.try_into().unwrap()
                ));
                height.try_into().unwrap()
            ]),
            overlay,
        );

        let mut lines = vec![
            Line::from(vec![
                Span::from("Name         "),
                Span::from(String::from(&app.activity_state.add.activity.name)),
            ]),
            Line::from(vec![
                Span::from("Color        "),
                Span::styled(
                    "■ ".to_owned()
                        + std::convert::Into::<&str>::into(app.activity_state.add.activity.color),
                    Style::default().fg(app.activity_state.add.activity.color.into()),
                ),
            ]),
            Line::from(vec![
                Span::from("Has exercise "),
                Span::from(match app.activity_state.add.activity.has_exercise {
                    true => "☒",
                    false => "☐",
                }),
            ]),
            Line::from("Save"),
        ];

        lines[app.activity_state.add.selected].spans[0].style =
            Style::default().fg(HIGHLIGHT_COLOR);

        frame.render_widget(
            tui::widgets::Paragraph::new(lines).block(
                Block::default()
                    .title("Add new activity")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(tui::style::Style::default().fg(settings::HIGHLIGHT_COLOR)),
            ),
            overlay,
        );
    }
}

const ADD_ACTIVITY_ROWS: usize = 4;
#[derive(Default)]
pub struct AddActivityState {
    selected: usize,
    pub activity: Activity,
}

impl AddActivityState {
    pub fn move_up(&mut self) {
        if self.selected == ADD_ACTIVITY_ROWS - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.selected == 0 {
            self.selected = ADD_ACTIVITY_ROWS - 1;
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
