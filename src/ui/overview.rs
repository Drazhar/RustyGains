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

    let heatmap_width: usize = (layout[1].width - 4).into();

    frame.render_widget(
        tui::widgets::Paragraph::new(crate::heatmap::create(heatmap_width)).block(
            Block::default()
                .title("Heatmap")
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
