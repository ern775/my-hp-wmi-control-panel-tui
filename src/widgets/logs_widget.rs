use ratatui::{
    buffer::Buffer,
    layout::Alignment,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

pub struct Logs<'a> {
    pub title: &'static str,

    pub y_scroll: u16,
    pub logs: Vec<Line<'a>>,
}

impl Logs<'_> {
    pub fn new(title: &'static str) -> Self {
        Self {
            title: title,
            y_scroll: 0,
            logs: vec![Line::from("Hello World!")],
        }
    }

    pub fn push_logs(&mut self, log: &str) {
        self.logs.push(Line::from(log));
    }
}

impl Widget for &Logs<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title);

        let logs = self.logs[..].to_vec();

        Paragraph::new(logs)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll((self.y_scroll, 0))
            .render(area, buf);
    }
}
