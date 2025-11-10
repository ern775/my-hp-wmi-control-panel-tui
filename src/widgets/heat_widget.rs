use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::Command,
};

use glob::glob;
use ratatui::{
    buffer::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
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

            res_str.trim().parse().expect(&format!(
                "failed to parse the output for get_cpu_temp : {} :",
                res_str
            ))
        } else {
            let err_msg = String::from_utf8_lossy(&temp_output.stderr);
            panic!("{}", err_msg);
        }
    }
}

impl Widget for &HeatWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title)
            .render(area, buf);

        Paragraph::new(format!("temp: {}", self.get_cpu_temp())).render(area, buf);
    }
}
