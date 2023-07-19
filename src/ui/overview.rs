mod big_nums;

use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Timelike;
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
use crate::settings::HIGHLIGHT_COLOR;

use self::big_nums::big_nums;

use super::log::ExerciseElement;
use super::log::LogArea;
use super::log::TimeSelection;
use super::{basic_layout, render_tabs};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.active_menu == Menu::Add {
        render_logging(app, frame);
    } else {
        let layout = basic_layout(frame);

        render_tabs(frame, app, layout[0]);

        let heatmap_width: usize = (layout[1].width - 4).into();

        let activity_log = app.db.get_activity_log(None);

        frame.render_widget(
            ratatui::widgets::Paragraph::new(crate::heatmap::create(heatmap_width, &app.db)).block(
                Block::default()
                    .title("Heatmap")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[1],
        );

        // TODO: Anpassen, dass man die Übungen sieht!
        let history_entries = {
            let mut result = Vec::with_capacity(activity_log.len());
            for a in &activity_log {
                let date = NaiveDateTime::from_timestamp_millis(a.date as i64 * 1000).unwrap();
                result.push(Row::new(vec![
                    Span::from(format!(
                        "{:02}.{:02}.{}:{:02}h",
                        date.day(),
                        date.month(),
                        date.year(),
                        date.hour()
                    )),
                    Span::styled(
                        &a.activity.name,
                        Style::default().fg(a.activity.color.into()),
                    ),
                    Span::from(format!("{}", a.intensity)),
                    Span::from(a.comment.clone()),
                ]));
            }
            result
        };
        app.log_state.table_size = history_entries.len();
        frame.render_stateful_widget(
            Table::new(history_entries)
                .header(
                    Row::new(["Date", "Activity", "Intensity", "Comment"]).style(
                        Style::new()
                            .fg(ratatui::style::Color::Yellow)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    ),
                )
                .widths(&[
                    Constraint::Length(15),
                    Constraint::Length(19),
                    Constraint::Length(2),
                    Constraint::Percentage(100),
                ])
                .highlight_symbol(">>")
                .highlight_style(if !activity_log.is_empty() {
                    Style::default().fg(activity_log[app.log_state.table.selected().unwrap()]
                        .activity
                        .color
                        .into())
                } else {
                    Style::default()
                })
                .block(
                    Block::default()
                        .title("Logs")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
            layout[2],
            &mut app.log_state.table,
        );
    }
}

fn render_logging<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let activities = &app.activity_state.activities;
    let selected_activity = &activities[app.log_state.selected_activity];

    let highlight_style = Style::default().fg(selected_activity.color.into());

    let width = 50;
    let multiline_comment = if app.log_state.comment.len() > width {
        app.log_state.comment.split_at(width)
    } else {
        (app.log_state.comment.as_str(), "")
    };

    let mut activity_menu = vec![
        Line::from(vec![
            Span::from("Activity:  "),
            Span::from(selected_activity.name.clone()),
        ]),
        Line::from(vec![
            Span::from("Date:      "),
            Span::from(format!("{:02}", app.log_state.date.day())),
            Span::from("."),
            Span::from(format!("{:02}", app.log_state.date.month())),
            Span::from("."),
            Span::from(format!("{}", app.log_state.date.year())),
            Span::from(" - "),
            Span::from(format!("{}", app.log_state.date.hour())),
            Span::from("h"),
        ]),
        Line::from(vec![
            Span::from("Intensity: "),
            Span::from(format!("{}", app.log_state.intensity)),
        ]),
        Line::from(format!("Comment:   {}", multiline_comment.0)),
        Line::from(format!("           {}", multiline_comment.1)),
    ];

    if app.log_state.active_area == LogArea::Activity {
        activity_menu[app.log_state.selected_activity_row].spans[0].style = highlight_style;

        let row = &app.log_state.selected_activity_row;
        match *row {
            0 => {
                activity_menu[*row].spans.insert(
                    1,
                    Span::styled("← ", highlight_style.add_modifier(Modifier::DIM)),
                );
                activity_menu[*row].spans[2].style = highlight_style;
                activity_menu[*row].spans.insert(
                    3,
                    Span::styled(" →", highlight_style.add_modifier(Modifier::DIM)),
                );
            }
            1 => {
                let highlight_index = match app.log_state.selected_time {
                    TimeSelection::Day => 1,
                    TimeSelection::Month => 3,
                    TimeSelection::Year => 5,
                    TimeSelection::Hour => 7,
                };

                activity_menu[*row].spans[highlight_index].style = highlight_style;
            }
            2 => {
                activity_menu[*row].spans[1].style = highlight_style;
            }
            3 => {
                activity_menu[*row + 1].spans[0].style = highlight_style;
            }
            _ => {}
        }
    }

    let elapsed_time = app.log_state.timer.get_elapsed();
    let break_time = app.log_state.timer.get_round();

    let big_nums = big_nums();
    let mut stopwatch = vec![Vec::new(); 5];
    for (i, sw) in stopwatch.iter_mut().enumerate() {
        sw.push(Span::from("        "));
        sw.extend(big_nums[break_time.num_minutes() as usize / 10][i].clone());
        sw.push(Span::from(" "));
        sw.extend(big_nums[break_time.num_minutes() as usize % 10][i].clone());
        sw.push(Span::from(" "));
        sw.extend(big_nums[10][i].clone());
        sw.push(Span::from(" "));
        sw.extend(big_nums[(break_time.num_seconds() % 60) as usize / 10][i].clone());
        sw.push(Span::from(" "));
        sw.extend(big_nums[(break_time.num_seconds() % 60) as usize % 10][i].clone());
    }
    stopwatch[0][0] = Span::from(format!(
        "{:02}:{:02}   ",
        elapsed_time.num_minutes(),
        elapsed_time.num_seconds() % 60
    ));

    let mut stopwatch_lines = Vec::new();
    for span in stopwatch {
        stopwatch_lines.push(Line::from(span));
    }

    let mut exercise_list = Vec::with_capacity(app.log_state.exercises.len() * 4 + 2);
    for exercise in &app.log_state.exercises {
        exercise_list.push(Line::from(vec![
            Span::from(exercise.exercise.name.clone()),
            Span::from(format!(" {:.1}kg", exercise.weight)),
            Span::from(format!(" {:.1}min", exercise.breaks)),
        ]));

        let mut rep_text = Vec::with_capacity(exercise.reps.len() + 2);
        rep_text.push(Span::from("  "));
        for r in &exercise.reps {
            rep_text.push(Span::from(format!("{},", r)));
        }
        rep_text.push(Span::styled(
            " +set",
            Style::default().add_modifier(Modifier::DIM),
        ));
        exercise_list.push(Line::from(rep_text));

        exercise_list.push(Line::from(vec![
            Span::from("  I"),
            Span::from(format!("{}", exercise.effort)),
        ]));
        exercise_list.push(Line::from(""));
    }
    exercise_list.push(Line::from(Span::styled(
        "<Add exercise>",
        Style::default().add_modifier(Modifier::DIM),
    )));

    if app.log_state.active_area == LogArea::Exercise {
        // Highlight selection
        let exercise_number = app.log_state.exercise_selection.exercise_number;
        let adjusted_index = exercise_number * 4;
        if exercise_number < app.log_state.exercises.len() {
            match app.log_state.exercise_selection.element {
                ExerciseElement::Name => {
                    exercise_list[adjusted_index].spans[0].style = highlight_style
                }
                ExerciseElement::Weight => {
                    exercise_list[adjusted_index].spans[1].style = highlight_style
                }
                ExerciseElement::Break => {
                    exercise_list[adjusted_index].spans[2].style = highlight_style
                }
                ExerciseElement::Set(i) => {
                    let spans = &mut exercise_list[adjusted_index + 1].spans;
                    let span = match spans.get_mut(i + 1) {
                        Some(s) => s,
                        None => {
                            app.log_state.exercise_selection.element = ExerciseElement::Set(0);
                            &mut spans[1]
                        }
                    };
                    span.style = highlight_style;
                }
                ExerciseElement::Intensity => {
                    exercise_list[adjusted_index + 2].spans[1].style = highlight_style
                }
            }
        } else {
            exercise_list[adjusted_index].spans[0].style = highlight_style;
        }
    }

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

    let mut activity_block = ratatui::widgets::Block::default()
        .title("Activity | Workout")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if app.log_state.active_area == LogArea::Activity {
        activity_block = activity_block.border_style(Style::default().fg(HIGHLIGHT_COLOR));
    }
    frame.render_widget(
        Paragraph::new(activity_menu).block(activity_block),
        upper_layout[0],
    );

    frame.render_widget(
        Paragraph::new(stopwatch_lines).block(
            ratatui::widgets::Block::default()
                .title("Stopwatch: s:start/stop - b:break - r:reset")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        upper_layout[1],
    );

    let mut exercise_block = ratatui::widgets::Block::default()
        .title("Exercises")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if app.log_state.active_area == LogArea::Exercise {
        exercise_block = exercise_block.border_style(Style::default().fg(HIGHLIGHT_COLOR));
    }
    frame.render_widget(
        Paragraph::new(exercise_list).block(exercise_block),
        lower_layout[0],
    );

    let data: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 4.0), (2.0, 0.8), (3.0, 0.8), (4.0, 4.0)];
    let datasets = vec![Dataset::default()
        .name("data")
        .graph_type(GraphType::Line)
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&data)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
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

    let history_title = match app.log_state.active_area {
        LogArea::Activity => "Activities",
        LogArea::Exercise => "Exercises",
    };

    let history_headers = match app.log_state.active_area {
        LogArea::Activity => vec!["Date", "Activity", "Intensity", "Comment"],
        LogArea::Exercise => vec!["Date", "Weight", "Break", "Sets", "Eff.", "Comment"],
    };

    let history_widths = match app.log_state.active_area {
        LogArea::Activity => vec![
            Constraint::Length(14),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Percentage(100),
        ],
        LogArea::Exercise => vec![
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(14),
            Constraint::Length(5),
            Constraint::Percentage(100),
        ],
    };

    let history_entries = match app.log_state.active_area {
        LogArea::Activity => {
            let activities = app.db.get_activity_log(Some(selected_activity.id));
            let mut result = Vec::with_capacity(activities.len());
            for a in activities {
                let date = NaiveDateTime::from_timestamp_millis(a.date as i64 * 1000).unwrap();
                result.push(Row::new(vec![
                    Span::from(format!(
                        "{:02}.{:02}.{} {:02}h",
                        date.day(),
                        date.month(),
                        date.year(),
                        date.hour()
                    )),
                    Span::from(a.activity.name.clone()),
                    Span::from(format!("{}", a.intensity)),
                    Span::from(a.comment),
                ]));
            }
            result
        }
        LogArea::Exercise => {
            if !app.log_state.exercises.is_empty()
                && app.log_state.exercise_selection.exercise_number < app.log_state.exercises.len()
            {
                let exercise_history = app.db.get_exercise_history(
                    app.log_state.exercises[app.log_state.exercise_selection.exercise_number]
                        .exercise
                        .id,
                );
                let mut result = Vec::with_capacity(exercise_history.len());

                for h in exercise_history {
                    result.push(Row::new(vec![
                        Span::from(format!(
                            "{:02}.{:02}.{} {:02}h",
                            h.date.day(),
                            h.date.month(),
                            h.date.year(),
                            h.date.hour()
                        )),
                        Span::from(format!("{}", h.weight)),
                        Span::from(format!("{}", h.breaks)),
                        Span::from(format!("{:?}", h.reps)),
                        Span::from(format!("{}", h.effort)),
                        Span::from(h.comment),
                    ]))
                }

                result
            } else {
                vec![]
            }
        }
    };

    frame.render_stateful_widget(
        Table::new(history_entries)
            .header(
                Row::new(history_headers).style(
                    Style::new()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            )
            .widths(&history_widths)
            .block(
                Block::default()
                    .title(history_title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        lower_layout_right[1],
        &mut app.activity_state.table,
    );
}
