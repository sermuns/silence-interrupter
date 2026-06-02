use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{Event as CrosstermEvent, KeyCode, KeyEventKind, KeyModifiers},
    },
    prelude::*,
    widgets::Widget,
};

pub struct App {
    running: bool,
    next_interruption: Instant,
}

fn random_future_instant() -> Instant {
    Instant::now() + Duration::from_secs(fastrand::u64(1..5))
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            next_interruption: random_future_instant(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> simple_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&*self, frame.area()))?;
            self.handle_user_input()?;

            if self.next_interruption.elapsed() > Duration::ZERO {
                println!("happened!");
                self.next_interruption = random_future_instant();
            }
        }

        Ok(())
    }

    fn handle_user_input(&mut self) -> io::Result<()> {
        if !crossterm::event::poll(Duration::from_millis(100))? {
            return Ok(());
        }
        let CrosstermEvent::Key(key_event) = crossterm::event::read()? else {
            return Ok(());
        };
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.running = false
            }
            _ => (),
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        "hehe".render(area, buf);
    }
}
