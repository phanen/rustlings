use crate::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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
        let pressed_key = Terminal::read_key()?;
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            // clear and redraw
            self.draw_rows(self.terminal.size().height as usize);
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    // clear and redraw
    fn draw_rows(&self, r: usize) {
        for row in 0..r - 1 {
            Terminal::clear_current_line();
            if row == r / 3 {
                self.draw_welcome_msg(self.terminal.size().width.into());
            } else {
                println!("~\r");
            }
        }
        print!("~");
    }

    fn draw_welcome_msg(&self, line_width: usize) {
        let welcome_msg = format!("Hecto editor -- version {}", VERSION);
        let max_width = std::cmp::max(line_width, welcome_msg.len());
        let pad_len = (max_width - welcome_msg.len()) / 2;
        println!(
            "~{}{}{}\r",
            " ".repeat(pad_len - 1),
            &welcome_msg,
            " ".repeat(pad_len)
        );
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
