use crate::Terminal;

use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to init terminal"),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Editor::read_key()?;
        match pressed_key {
            // by default, it will print to io buffer
            Key::Char(c) => print!("{}\r", c as u8),
            // two hack ways to flush, more standardly should directly call `stdout().flush()`
            // Key::Char(c) => print!("{}\n", c as u8),
            // Key::Char(c) => println!("{}", c as u8),
            Key::Ctrl('q') => self.should_quit = true,
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // protocol? CSI sequence
        // print!("\x1b[2J");
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows(24 as usize);
        }
        io::stdout().flush()
    }

    fn draw_rows(&self, r: usize) {
        for _ in 0..r {
            println!("~\r");
        }
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
