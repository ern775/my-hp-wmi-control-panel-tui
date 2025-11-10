use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct Navbar {
    pub title: &'static str,
}

impl Navbar {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }
}

impl Widget for &Navbar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .border_type(BorderType::Rounded)
            .render(area, buf);
    }
}
