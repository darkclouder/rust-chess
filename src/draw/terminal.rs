use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

pub struct Terminal {
    pub screen: AlternateScreen<io::Stdout>,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Self {
        Self {
            screen: AlternateScreen::from(io::stdout()),
            _stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn read_key_raw(&self) -> Option<Key> {
        let result = io::stdin().lock().keys().next();

        match result {
            Some(Ok(key)) => Some(key),
            _ => None,
        }
    }

    pub fn read_key(&self) -> Option<Key> {
        let key = self.read_key_raw();

        match key {
            Some(Key::Ctrl('c')) => panic!("Program end"),
            _ => key,
        }
    }

    pub fn clear_screen(&mut self) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        write!(
            self.screen,
            "{}",
            termion::cursor::Goto(
                x.saturating_add(1).try_into().unwrap(),
                y.saturating_add(1).try_into().unwrap(),
            ),
        )
        .unwrap();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
    }
}
