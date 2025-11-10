use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::Command,
};

use glob::glob;
use ratatui::{
    buffer::Buffer,
    layout::Direction,
    style::{Color, Style, Styled, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, BorderType, Borders, Chart, Dataset, GraphType,
        Padding, Paragraph, Widget,
    },
};

pub struct HeatWidget {
    pub title: &'static str,
}

impl HeatWidget {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }

    pub fn get_cpu_temp(&self) -> u64 {
        let temp_output = Command::new("sh")
            .arg("-c")
            .arg("grep k10temp /sys/class/hwmon/hwmon*/name")
            .output()
            .expect("Cloudnt run grep");

        if temp_output.status.success() {
            let grep_res_str = String::from_utf8_lossy(&temp_output.stdout);

            // gonna return something like
            // /sys/class/hwmon/hwmon5/name:k10temp
            let temp_path = grep_res_str.lines().next().unwrap_or("").trim();

            // This should return
            // /sys/class/hwmon/hwmon5
            let parent: String = Path::new(temp_path.split_once(':').unwrap().0)
                .parent()
                .unwrap()
                .display()
                .to_string();

            let path = glob(&format!("{}/temp1_input", &parent))
                .expect("failed to read glob pattern")
                .filter_map(Result::ok)
                .next()
                .expect(&format!("no match cpu found also: {}", &parent));

            let res_str = fs::read_to_string(&path).expect("failed to read temp_input");

            if res_str.is_empty() {
                panic!("cpu returned empty temp")
            }

            let degrees: u64 = res_str.trim().parse().expect(&format!(
                "failed to parse the output for get_cpu_temp : {} :",
                res_str
            ));
            degrees / 1000
        } else {
            let err_msg = String::from_utf8_lossy(&temp_output.stderr);
            panic!("{}", err_msg);
        }
    }

    pub fn temperature_style(&self, value: u8) -> Style {
        let green = (255.0 * (1.0 - f64::from(value.saturating_sub(50)) / 40.0)) as u8;
        let color = Color::Rgb(255, green, 0);
        Style::new().fg(color)
    }
}

impl Widget for &HeatWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title)
            .padding(Padding::uniform(1));

        // Create the datasets to fill the chart with
        let temp1 = self.get_cpu_temp();
        let temp2 = 55;

        let style1 = self.temperature_style(temp1 as u8);
        let style2 = self.temperature_style(temp2 as u8);

        let bars: Vec<Bar> = vec![
            Bar::default()
                .value(temp1)
                .label(Line::from("temp1"))
                .text_value(format!("{}C", temp1))
                .style(style1)
                .value_style(style1.reversed()),
            Bar::default()
                .value(temp2)
                .label(Line::from("temp2"))
                .text_value(format!("{}C", temp2))
                .style(style2)
                .value_style(style2.reversed()),
        ];

        BarChart::default()
            .block(block)
            .data(BarGroup::default().bars(&bars))
            .bar_width(10)
            .bar_gap(2)
            .direction(Direction::Vertical)
            .render(area, buf);
    }
}
