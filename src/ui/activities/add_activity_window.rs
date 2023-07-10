use tui::{
    backend::Backend,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, data::Activity, settings, ui::floating_window};

const ADD_ACTIVITY_ROWS: usize = 4;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let width = 60;
    let height = ADD_ACTIVITY_ROWS as u16 + 2;
    let overlay = floating_window::create(frame, width, height);

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

    if !lines.is_empty() {
        lines[app.activity_state.add.selected].spans[0].style =
            Style::default().fg(settings::HIGHLIGHT_COLOR);
    }

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