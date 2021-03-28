// taken from tui-rs examples

use std::{io, time::Instant};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode};

pub enum Event<I> {
    Input(I),
    Tick,
}
pub struct Events {
    rx: mpsc::Receiver<Event<KeyCode>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: KeyCode::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Self {
        Self::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                if event::poll(config.tick_rate).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key.code)).unwrap();
                    }
                }
            }
        });

        Self { rx }
    }

    pub fn next(&self) -> Result<Event<KeyCode>, mpsc::RecvError> {
        self.rx.recv()
    }
}