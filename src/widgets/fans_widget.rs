use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    process::Command,
};

use glob::glob;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct Fan {
    pub id: usize,
}

// id gonna be hardcoded for now
impl Fan {
    pub fn new(id: usize) -> Self {
        Self { id: id }
    }

    pub fn get_max_fan_speed(&self) -> io::Result<u16> {
        let pattern = format!(
            "/sys/devices/platform/hp-wmi/hwmon/hwmon*/fan{}_max",
            self.id
        );

        let path = glob(&pattern)
            .expect("failed to read fan_max")
            .filter_map(Result::ok)
            .next()
            .expect("no fan max file found");

        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let number: u16 = contents
            .trim()
            .parse()
            .expect("failed to parse the number from fan_max");

        Ok(number)
    }

    pub fn set_target_speed(&self, mut speed: u16) -> io::Result<()> {
        let max_speed = self.get_max_fan_speed()?;

        if speed > max_speed {
            speed = max_speed
        }

        let pattern = format!(
            "/sys/devices/platform/hp-wmi/hwmon/hwmon*/fan{}_target",
            self.id
        );

        let path = glob(&pattern)
            .expect("failed to read glob pattern")
            .filter_map(Result::ok)
            .next()
            .expect("no matching fan target file found");

        let mut file = OpenOptions::new().write(true).open(path)?;
        let data = format!("{}", speed);

        file.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn get_cur_speed(&self) -> u16 {
        let pattern = format!(
            "/sys/devices/platform/hp-wmi/hwmon/hwmon*/fan{}_input",
            self.id
        );

        let path = glob(&pattern)
            .expect("failed to read glob pattern")
            .filter_map(Result::ok)
            .next()
            .expect("no matching fan input file found");

        let res_str = fs::read_to_string(&path)
            .expect("failed to read fan input")
            .trim()
            .to_string();

        if res_str.is_empty() {
            panic!("fan {} returned empty speed", self.id);
        }

        res_str
            .parse()
            .expect("failed to parse the output for get_cur_speed")
    }
}

impl Widget for &Fan {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Current Speed of fan_{} : {}",
            self.id,
            self.get_cur_speed()
        ))
        .render(area, buf);
    }
}

pub struct FansWidget {
    pub title: &'static str,
}

impl FansWidget {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }
}

impl Widget for &FansWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let layout = Layout::vertical(vec![Constraint::Fill(1), Constraint::Fill(1)]).split(area);
        let (visuals_area, data_area) = (layout[0], layout[1]);

        let fan1 = Fan::new(1);
        let fan2 = Fan::new(2);

        let vis_area_lay =
            Layout::horizontal(vec![Constraint::Fill(1), Constraint::Fill(1)]).split(visuals_area);

        fan1.render(vis_area_lay[0], buf);
        fan2.render(vis_area_lay[1], buf);
    }
}
