use crate::Document;
use crate::Row;
use crate::Terminal;

use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    // 0-indexed
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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
        // let args: Vec<String> = env::args().collect();
        //
        // let document = if args.len() > 1 {
        //     let file_name = &args[1];
        //     Document::open(file_name).unwrap_or_default()
        // } else {
        //     Document::default()
        // };

        env::args().next().unwrap();
        let filename = env::args().next().unwrap_or_default();
        let document = Document::open(&filename).unwrap_or_default();

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to init terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        // update cursor
        match pressed_key {
            // by default, it will print to io buffer
            // Key::Char(c) => print!("{}\r", c as u8),
            Key::Char('h')
            | Key::Char('j')
            | Key::Char('k')
            | Key::Char('l')
            | Key::Char('H')
            | Key::Char('L')
            | Key::Char('^')
            | Key::Char('$') => self.move_cursor(pressed_key),
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        // scroll or not based on cursor position
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height as usize;
        let width = size.width as usize;

        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        // we need 0-index form here
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Char('h') => x = x.saturating_sub(1),
            Key::Char('j') => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('l') => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Char('H') => y = 0,
            Key::Char('L') => y = height,
            Key::Char('^') => x = 0,
            Key::Char('$') => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }

    // update cursor position
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            // clear and redraw
            self.draw_rows(self.terminal.size().height as usize);
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    // clear and redraw
    fn draw_rows(&self, rows: usize) {
        // if self.document.is_empty() {
        //     for row_id in 0..rows - 1 {
        //         Terminal::clear_current_line();
        //         if row_id == rows / 3 {
        //             self.draw_welcome_msg(self.terminal.size().width.into());
        //         } else {
        //             println!("~\r");
        //         }
        //     }
        // }

        for row_id in 0..rows - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(self.offset.y + row_id) {
                self.draw_row(row)
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
