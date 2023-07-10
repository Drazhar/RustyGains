mod activities;
mod exercises;
mod floating_window;
mod overview;
pub mod tabs;
mod workouts;

pub use activities::add_activity_window::AddActivityState;
pub use activities::ActivityState;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, settings};

use self::tabs::Tab;

fn basic_layout<B: Backend>(frame: &mut Frame<B>) -> std::rc::Rc<[tui::layout::Rect]> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(12),
                Constraint::Percentage(1),
            ]
            .as_ref(),
        )
        .split(frame.size());
    layout
}

fn render_tabs<B: Backend>(frame: &mut Frame<B>, app: &App, area: Rect) {
    let tab_titles = Tab::line_vec();
    frame.render_widget(
        tui::widgets::Tabs::new(tab_titles)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                    .border_type(BorderType::Rounded),
            )
            .select(app.active_tab.as_usize())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(settings::HIGHLIGHT_COLOR))
            .divider("|"),
        area,
    );
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.active_tab {
        Tab::Overview => overview::render(app, frame),
        Tab::Exercises => exercises::render(app, frame),
        Tab::Workouts => workouts::render(app, frame),
        Tab::Activities => activities::render(app, frame),
    }
}
