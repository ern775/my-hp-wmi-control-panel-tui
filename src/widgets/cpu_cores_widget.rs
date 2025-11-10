use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct CoresWidget {
    pub title: &'static str,
}

impl CoresWidget {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }
}

impl Widget for &CoresWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let title = Line::from(format!("{}", self.title).bold());

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title)
            .render(area, buf);
    }
}
