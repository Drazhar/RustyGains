use tui::{
    backend::Backend,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::app::App;

use super::{basic_layout, render_tabs};

pub fn render<B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    let layout = basic_layout(frame);

    render_tabs(frame, app, layout[0]);

    frame.render_widget(
        tui::widgets::Paragraph::new("").block(
            Block::default()
                .title("Exercises")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        layout[1],
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
}
