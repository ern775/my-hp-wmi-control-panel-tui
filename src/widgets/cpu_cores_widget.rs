use ratatui::{
    buffer::Buffer,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Widget},
};
use sysinfo::{Cpu, System};

use crate::margin;

pub struct CoresWidget {
    pub title: &'static str,
    sys: System,
}

impl CoresWidget {
    pub fn new(title: &'static str) -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu();

        Self { title, sys }
    }

    // ---------------------------------------------------------
    // IMPORTANT: Call this function in your main app loop!
    // e.g. every tick or every 500ms.
    // ---------------------------------------------------------
    pub fn update(&mut self) {
        self.sys.refresh_cpu();
    }

    pub fn mk_bars(&self) -> Vec<Bar<'_>> {
        self.sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                let usage_percent = cpu.cpu_usage(); // Returns f32 (0.0 to 100.0)

                let bar_value = ((usage_percent / 10.0) as u64).max(1);

                Bar::default()
                    .value(bar_value)
                    .label(Line::from(format!("cpu{}", i)))
                    .text_value("".to_string())
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

