use crate::Document;
use crate::Row;
use crate::Terminal;

use std::env;
use termion::color;
use termion::event::Key;

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
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
            if self.should_quit {
                break;
            }

            // render screen based on current state
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            // get key, update to next state
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

        let mut args = env::args();
        args.next().unwrap();
        let filename = args.next().unwrap_or_default();
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
            | Key::Ctrl('u')
            | Key::Ctrl('d')
            | Key::Char('^')
            | Key::Char('$') => self.move_cursor(pressed_key),
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        // scroll or not, based on cursor position
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height as usize;
        let width = size.width as usize;

        let offset = &mut self.offset;
        if y >= offset.y.saturating_add(height) {
            // down overflow
            offset.y = y.saturating_sub(height).saturating_add(1);
        } else if y < offset.y {
            // up overflow
            offset.y = y;
        }

        if x >= offset.x.saturating_add(width) {
            // right overflow
            offset.x = x.saturating_sub(width).saturating_add(1);
        } else if x < offset.x {
            // left overflow
            offset.x = x;
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        // let size = self.terminal.size();
        // let height = size.height.saturating_sub(1) as usize;
        // let width = size.width.saturating_sub(1) as usize;
        let height = self.document.len();
        let t_height = self.terminal.size().height as usize;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Char('h') => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    // linewrap back
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Char('j') => {
                if y < height - 1 {
                    y = y.saturating_add(1);
                }
            }
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('l') => {
                if x == width {
                    if y < height - 1 {
                        y += 1;
                        x = 0;
                    }
                } else if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Ctrl('u') => y = if y > t_height { y - t_height } else { 0 },
            Key::Ctrl('d') => {
                y = if y.saturating_add(t_height) < height - 1 {
                    y + t_height as usize
                } else {
                    height - 1
                }
            }
            Key::Char('^') => x = 0,
            Key::Char('$') => x = width,
            _ => (),
        }

        // limit x in new width (y change -> width change)
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width
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
            // clear and redraw (make a absoulte cursor pos)
            self.draw_rows(self.terminal.size().height as usize);
            self.draw_status_bar();
            self.draw_msg_bar();
            // transform absoulte cursor pos back to relative cursor pos
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row, no_break: bool) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        // if no_break {
        //     print!("{}\r", row);
        // } else {
        println!("{}\r", row);
        // }
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

        for row_id in 0..rows {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(self.offset.y.saturating_add(row_id)) {
                self.draw_row(row, row_id == rows - 1);
            } else {
                if row_id == rows - 1 {
                    print!("~\r");
                } else {
                    println!("~\r");
                }
            }
        }
    }

    fn draw_status_bar(&self) {
        let spaces = " ".repeat(self.terminal.size().width as usize);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        println!("{}\r", spaces);
        Terminal::reset_bg_color();
    }
    fn draw_msg_bar(&self) {
        Terminal::clear_current_line();
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
