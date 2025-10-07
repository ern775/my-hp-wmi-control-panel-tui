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

use crate::widgets::{counter::Counter, fans_widget::FansWidget};

pub struct App {
    pub exit: bool,

    // widgets:
    pub counter: Counter,
    pub fans_widget: FansWidget,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_update = Instant::now();

        while !self.exit {
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
            .horizontal_margin(20)
            .vertical_margin(5)
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let (_header, fans_area, _area_2) = (main_layout[0], main_layout[1], main_layout[2]);

        frame.render_widget(&self.fans_widget, fans_area);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            counter: Counter {
                title: "X",
                count: 0,
            },
            fans_widget: FansWidget::new("Fans"),
        }
    }
}
