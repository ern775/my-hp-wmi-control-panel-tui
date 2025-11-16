use std::{
    env::args_os,
    fs::{self, OpenOptions},
    path::Path,
    process::{Command, Stdio},
};

use glob::glob;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Styled, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, BorderType, Borders, Chart, Dataset, GraphType,
        Padding, Paragraph, Widget,
    },
};

use crate::margin;

pub struct UsageWidget {
    pub title: &'static str,
}

impl UsageWidget {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }

    // GPU STUFF HERE
    pub fn is_gpu_active(&self) -> bool {
        Command::new("sh")
            .arg("-c")
            .arg("nvidia-smi -L")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Couldnt run nvidia-smi")
            .success()
    }

    pub fn get_gpu_heat(&self) -> u8 {
        if (self.is_gpu_active()) {
            let output = Command::new("sh")
                .arg("-c")
                .arg("nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits")
                .output()
                .expect("couldnt run nvidia-smi gpu-heat")
                .stdout;

            String::from_utf8(output)
                .unwrap()
                .trim()
                .parse()
                .expect("Couldnt parse")
        } else {
            // return 0 if not active,
            // gpu heat cant be 0 anyway.
            0
        }
    }

    // CPU STUFF HERE
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
        let green = (255.0 * (1.0 - f64::from(value.saturating_sub(55)) / 40.0)) as u8;
        let color = Color::Rgb(255, green, 0);
        Style::new().fg(color)
    }

    pub fn get_ram_usage(&self) -> (u64, u64) {
        let ram_output = Command::new("sh")
            .arg("-c")
            .arg("free -m | awk 'NR==2{print $2, $3}'")
            .output()
            .expect("Couldnt run free -m");

        let ram_output_str = String::from_utf8(ram_output.stdout).unwrap().to_string();

        let str_split = ram_output_str.trim().split_once(" ").unwrap();
        let (total, used) = (
            str_split.0.parse::<u64>().unwrap(),
            str_split
                .1
                .parse::<u64>()
                .expect(&format!("{:?}", str_split)),
        );

        (total, used)
    }
}

impl Widget for &UsageWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.title);

        let heat_block = Block::default()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);

        // Create the datasets to fill the chart with
        let cpu_temp = self.get_cpu_temp();
        let gpu_temp = self.get_gpu_heat();

        let cpu_temp_style = self.temperature_style(cpu_temp as u8);
        let gpu_temp_style = self.temperature_style(gpu_temp as u8);

        let bars: Vec<Bar> = vec![
            Bar::default()
                .value(cpu_temp)
                .label(Line::from("CPU"))
                .text_value(format!(" {}ºC", cpu_temp))
                .style(cpu_temp_style)
                .value_style(cpu_temp_style.reversed()),
            Bar::default()
                .value(gpu_temp as u64)
                .label(Line::from("GPU"))
                .text_value(format!(" {}ºC", gpu_temp))
                .style(gpu_temp_style)
                .value_style(gpu_temp_style.reversed()),
        ];

        main_block.render(area, buf);

        let layout_main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(20), Constraint::Fill(2)])
            .split(area.inner(margin!(2, 1)));

        BarChart::default()
            .block(heat_block)
            .data(BarGroup::default().bars(&bars))
            .max(100)
            .bar_width(8)
            .bar_gap(2)
            .direction(Direction::Vertical)
            .render(layout_main[0], buf);

        let (total, used): (u64, u64) = self.get_ram_usage();
        let total_ram_gib = total as f64 / 1024.0;
        let used_ram_gib = used as f64 / 1024.0;

        let bars: Vec<Bar> = vec![
            Bar::default()
                .value(used)
                .label(Line::from("RAM"))
                .text_value(format!("{:.1}GiB", used_ram_gib)),
        ];
        BarChart::default()
            .direction(Direction::Horizontal)
            .bar_width(5)
            .bar_gap(2)
            .data(BarGroup::default().bars(&bars))
            .max(total)
            .render(layout_main[1], buf);
    }
}
