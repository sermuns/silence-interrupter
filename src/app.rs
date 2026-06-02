use std::{
    io::{self},
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
use rodio::{MixerDeviceSink, Player};

use crate::audio::get_random_audio;

pub struct App {
    running: bool,
    next_interruption: Instant,
    audio_device: MixerDeviceSink,
    audio_player: Player,
}

fn random_future_instant() -> Instant {
    Instant::now() + Duration::from_secs(fastrand::u64(1..5))
}

impl App {
    pub fn new() -> simple_eyre::Result<Self> {
        let audio_device = rodio::DeviceSinkBuilder::open_default_sink()?;
        let audio_player = Player::connect_new(audio_device.mixer());

        Ok(Self {
            running: true,
            next_interruption: random_future_instant(),
            audio_device,
            audio_player,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> simple_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&*self, frame.area()))?;
            self.handle_user_input()?;

            if self.next_interruption.elapsed() > Duration::ZERO {
                println!("happened!");
                self.audio_player = rodio::play(self.audio_device.mixer(), get_random_audio())?;
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
