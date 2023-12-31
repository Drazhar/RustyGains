use ratatui::{
    backend::Backend,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Wrap},
    Frame,
};

use crate::{
    app::{App, Menu},
    ui::floating_window,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let width = 50;
    let height = 12;
    let overlay = floating_window::create(frame, width, height);

    let selected_row = &app.activity_state.table.selected();

    if selected_row.is_none() {
        app.active_menu = Menu::Main;
    }

    let activity = &app.activity_state.activities[selected_row.unwrap()];

    let lines = vec![
        Line::from(vec![
            Span::from("Are you sure you want to delete the activity "),
            Span::styled(&activity.name, Style::default().fg(activity.color.into())),
            Span::from("? This will also delete ALL log entries of this activity!"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::from("To confirm, please type '"),
            Span::styled(
                &activity.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::from("'. Press "),
            Span::styled("esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(" to cancel."),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            &app.activity_state.delete_confirm,
            Style::default()
                .fg(ratatui::style::Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    if app.activity_state.delete_confirm == activity.name {
        app.delete_activity();
        app.activity_state.delete_confirm.truncate(0);
        app.active_menu = Menu::Main;
    } else {
        frame.render_widget(
            ratatui::widgets::Paragraph::new(lines)
                .alignment(ratatui::layout::Alignment::Center)
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title(format!("Delete activity: {}", activity.name))
                        .padding(Padding::uniform(1))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(ratatui::style::Style::default().fg(activity.color.into())),
                ),
            overlay,
        );
    }
}
