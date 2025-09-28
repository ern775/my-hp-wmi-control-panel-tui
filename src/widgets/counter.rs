use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct Counter {
    pub title: &'static str,
    pub count: u8,
}

impl Counter {
    pub fn new(title: &'static str) -> Self {
        Self {
            title: title,
            count: 0,
        }
    }

    pub fn increase_counter(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    pub fn decrease_counter(&mut self) {
        self.count = self.count.saturating_sub(1);
    }
}

impl Widget for &Counter {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let title = Line::from(format!("Counter '{}': {}", self.title, self.count).bold());

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new(title)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
