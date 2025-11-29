use std::{
    io,
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
};

use crate::widgets::{
    counter::Counter, cpu_cores_widget::CoresWidget, fans_widget::FansWidget, logs_widget::Logs,
    navbar::Navbar, usage_widget::UsageWidget,
};

pub struct App<'a> {
    pub exit: bool,

    // widgets:
    pub counter: Counter,

    pub navbar: Navbar,
    pub fans_widget: FansWidget,
    pub cores_widget: CoresWidget,

    pub usage_widget: UsageWidget,
    pub logs_widget: Logs<'a>,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_update = Instant::now();

        let mut last_cpu_update = Instant::now();

        self.cores_widget.update();
        self.usage_widget.update_cpu_data();

        while !self.exit {
            let now = Instant::now();
            if last_cpu_update.elapsed() >= Duration::from_millis(200) {
                self.cores_widget.update();
                self.usage_widget.update_cpu_data();
                last_cpu_update = now;
            }

            // Always redraw every 1000 / 30 = 33 milliseconds
            if last_update.elapsed() >= Duration::from_millis(33) {
                terminal.draw(|frame| self.draw(frame))?;
                last_update = Instant::now();
            }

            if event::poll(Duration::from_millis(0))? {
                self.handle_events()?;
            }

            sleep(Duration::from_millis(1));
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.render(frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('q'), _) => self.exit(),
            (KeyCode::Up, _) => self.counter.increase_counter(),
            (KeyCode::Down, _) => self.counter.decrease_counter(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(2), Constraint::Fill(1)])
            .split(frame.area());

        let (left_col, right_col) = (main_layout[0], main_layout[1]);

        let left_col_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(10),
            ])
            .split(left_col);

        let (navbar_area, fanwidget_area, core_usage_area) =
            (left_col_layout[0], left_col_layout[1], left_col_layout[2]);

        let right_col_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(3), Constraint::Fill(2)])
            .split(right_col);

        let (heat_area, logs_area) = (right_col_layout[0], right_col_layout[1]);

        frame.render_widget(&self.navbar, navbar_area);
        frame.render_widget(&self.fans_widget, fanwidget_area);
        frame.render_widget(&self.cores_widget, core_usage_area);

        frame.render_widget(&self.usage_widget, heat_area);
        frame.render_widget(&self.logs_widget, logs_area);
    }


    fn push_log(&mut self) {
        self.logs_widget.
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            exit: false,
            counter: Counter {
                title: "X",
                count: 0,
            },

            navbar: Navbar::new("Navbar"),
            fans_widget: FansWidget::new("Fans"),
            cores_widget: CoresWidget::new("Cores"),

            usage_widget: UsageWidget::new("Usage"),
            logs_widget: Logs::new("Logs"),
        }
    }
}
