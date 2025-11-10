use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct Logs {
    pub title: &'static str,
}

impl Logs {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }
}

impl Widget for &Logs {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let title = Line::from(format!("{}", self.title).bold());

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title)
            .render(area, buf);
    }
}
