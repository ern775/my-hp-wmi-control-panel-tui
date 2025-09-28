use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::widgets::counter::Counter;

pub struct App {
    pub exit: bool,

    // widgets:
    pub counter: Counter,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
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
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let (counter_area, _area_1, _area_2) = (main_layout[0], main_layout[1], main_layout[2]);

        frame.render_widget(&self.counter, counter_area);
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
        }
    }
}
