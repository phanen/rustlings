use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = Editor::read_key()?;
        match pressed_key {
            Key::Char(c) => println!("{}\r", c as u8),
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}
