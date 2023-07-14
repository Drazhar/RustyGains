use ratatui::symbols::Marker;
use ratatui::widgets::GraphType;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Paragraph},
    Frame,
};

use crate::app::{App, Menu};

use super::{basic_layout, render_tabs};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.active_menu == Menu::Add {
        render_logging(app, frame);
    } else {
        let layout = basic_layout(frame);

        render_tabs(frame, app, layout[0]);

        let heatmap_width: usize = (layout[1].width - 4).into();

        frame.render_widget(
            ratatui::widgets::Paragraph::new(crate::heatmap::create(heatmap_width)).block(
                Block::default()
                    .title("Heatmap")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[1],
        );
        frame.render_widget(
            ratatui::widgets::Paragraph::new("").block(
                Block::default()
                    .title("Logs")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[2],
        );
    }
}

fn render_logging<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let activities = &app.activity_state.activities;

    let activity_menu = vec![
        Line::from("Activity:  ← Bla →"),
        Line::from("Date:      12.05.2015"),
        Line::from("Intensity: 4"),
        Line::from("Comment:   blablabl"),
        Line::from("           asdfasdfasdfasdf"),
    ];

    let stopwatch = vec![
        Line::from("51:23    0000  1111      2222   3333 "),
        Line::from("        00  00   11   : 22  22 33  33"),
        Line::from("08:56   00  00   11        22     33"),
        Line::from("02:20   00  00   11   :   22   33  33"),
        Line::from("01:50    0000  111111   222222  3333"),
    ];

    let exercises = vec![
        Line::from("Klimmzüge  5kg  3min"),
        Line::from("  5,5,5,5,5"),
        Line::from("  C: Hart heute"),
        Line::from(""),
        Line::from("Liegestütz  0kg  2min"),
        Line::from("  14,14,14"),
        Line::from("  C: Mittel"),
        Line::from(""),
        Line::from("Hängen  7.5kg  3min"),
        Line::from("  10,10,10,10,10"),
        Line::from("  C: Mittel"),
        Line::from(""),
        Line::from(Span::styled(
            "<Add>",
            Style::default().add_modifier(Modifier::DIM),
        )),
    ];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let upper_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Length(40)].as_ref())
        .split(layout[0]);

    let lower_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(32), Constraint::Percentage(50)].as_ref())
        .split(layout[1]);

    let lower_layout_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(lower_layout[1]);

    frame.render_widget(
        Paragraph::new(activity_menu).block(
            ratatui::widgets::Block::default()
                .title("Activity | Workout")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        upper_layout[0],
    );
    frame.render_widget(
        Paragraph::new(stopwatch).block(
            ratatui::widgets::Block::default()
                .title("Stopwatch: s:start/stop - b:break - r:reset")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        upper_layout[1],
    );
    frame.render_widget(
        Paragraph::new(exercises).block(
            ratatui::widgets::Block::default()
                .title("Exercises")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        lower_layout[0],
    );

    let data: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
    let datasets = vec![Dataset::default()
        .name("data")
        .graph_type(GraphType::Line)
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&data)];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart 1").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .bounds([0.0, 4.0])
                .labels(vec!["0.0".into(), "4.0".into()])
                .style(Style::default().fg(Color::Gray)),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, 4.0])
                .style(Style::default().fg(Color::Gray))
                .labels(vec!["0.0".into(), "4.0".into()]),
        );
    frame.render_widget(chart, lower_layout_right[0]);

    frame.render_stateful_widget(
        Table::new(vec![Row::new(vec![
            Span::from("12.03.45"),
            Span::from("7.5 kg"),
            Span::from("3:00"),
            Span::from("4x5,4"),
            Span::from("9"),
            Span::from("Grenzwertig"),
        ])])
        .header(
            Row::new(vec!["Date", "Weight", "Break", "Sets", "Eff.", "Comment"]).style(
                Style::new()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
        )
        .widths(&[
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(5),
            Constraint::Percentage(100),
        ])
        .highlight_style(if !app.activity_state.activities.is_empty() {
            Style::default().fg(app.activity_state.activities
                [app.activity_state.table.selected().unwrap_or(0)]
            .color
            .into())
        } else {
            Style::default()
        })
        .block(
            Block::default()
                .title("Activities")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        lower_layout_right[1],
        &mut app.activity_state.table,
    );
}
