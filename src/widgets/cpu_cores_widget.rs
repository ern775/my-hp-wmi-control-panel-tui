use std::process::Command;

use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Paragraph, Widget},
};

use crate::margin;

pub struct CoresWidget {
    pub title: &'static str,
}

impl CoresWidget {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }

    pub fn get_cpu_count(&self) -> u8 {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lscpu | grep '^CPU(s):' | awk '{print $2}'")
            .output()
            .expect("Couldnt run lscpu");

        String::from_utf8(output.stdout)
            .unwrap()
            .trim()
            .parse()
            .expect("Couldnt parse")
    }

    // this is cool
    pub fn mk_bars(&self) -> Vec<Bar<'_>> {
        (0..self.get_cpu_count())
            .map(|i| {
                Bar::default()
                    .value(0)
                    .label(Line::from(format!("cpu{}", i)))
                    .text_value(String::from(""))
            })
            .collect()
    }
}

impl Widget for &CoresWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title);

        let bars = self.mk_bars();

        BarChart::default()
            .block(block)
            .data(BarGroup::default().bars(&bars))
            .max(10)
            .bar_width(6)
            .bar_gap(2)
            .render(area.inner(margin!(2, 1)), buf);
    }
}
