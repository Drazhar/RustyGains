use ratatui::{backend::Backend, layout::Rect, text::Line, Frame};

pub fn create<B: Backend>(frame: &mut Frame<B>, mut width: u16, mut height: u16) -> Rect {
    let mut x = 0;
    let mut y = 0;

    // Center the floating window
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

    // Fill the layout with whitespace
    let overlay = Rect::new(x, y, width, height);
    frame.render_widget(
        ratatui::widgets::Paragraph::new(vec![
            Line::from(str::repeat(
                " ",
                width.try_into().unwrap()
            ));
            height.try_into().unwrap()
        ]),
        overlay,
    );

    overlay
}
