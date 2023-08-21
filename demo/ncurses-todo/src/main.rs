use ncurses::*;
use std::fs::File;
use std::io::Write;
// use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed!");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list elements outside of lists!");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });
    }

    fn end_list(&mut self) {
        self.list_curr = None
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end(&mut self) {
        todo!()
    }
}

enum Status {
    Todo,
    Done,
}

impl Status {
    fn toggle(self) -> Self {
        match self {
            Self::Todo => Self::Done,
            Self::Done => Self::Todo,
        }
    }
}

// fn parse_item() -> Option<(Status, &str)> {
//     todo!()
// }

fn list_up(_list: &mut Vec<String>, list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

fn list_down(list: &mut Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
    }
}

fn list_transfer(
    list_dst: &mut Vec<String>,
    list_src: &mut Vec<String>,
    list_src_curr: &mut usize,
) {
    if *list_src_curr < list_src.len() {
        list_dst.push(list_src.remove(*list_src_curr));
    }
    // ensure the cursor to be valid
    if list_src.len() != 0 && *list_src_curr == list_src.len() {
        *list_src_curr -= 1;
    }
}

// TODO: add/delete/edit items
// TODO: load/save config
fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut todos: Vec<String> = Vec::<String>::new();
    let mut dones: Vec<String> = Vec::<String>::new();

    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;

    let mut ui = Ui::default();

    let mut status = Status::Todo;

    loop {
        erase();
        ui.begin(0, 0);

        match status {
            Status::Todo => {
                ui.label("[TODO]  DONE ", REGULAR_PAIR);
                ui.begin_list(todo_curr);
                for (index, todo) in todos.iter().enumerate() {
                    ui.list_element(&format!("- [ ] {}", todo), index);
                }
                ui.end_list();
            }
            Status::Done => {
                ui.label(" TODO  [DONE]", REGULAR_PAIR);
                ui.begin_list(done_curr);
                for (index, done) in dones.iter().enumerate() {
                    ui.list_element(&format!("- [x] {}", done), index);
                }
                ui.end_list();
            }
        }
        // ui.label("--------------------------------", REGULAR_PAIR);
        refresh();

        // key handler
        let key = getch();
        match key as u8 as char {
            'q' => break,
            // 'e' => {
            //     let mut file = File::create("TODO").unwrap();
            //     for todo in todos.iter() {
            //         writeln!(file, "TODO: {}", todo).unwrap();
            //     }
            //     for done in dones.iter() {
            //         writeln!(file, "DONE: {}", done).unwrap();
            //     }
            // }
            'k' => match status {
                Status::Todo => list_up(&mut todos, &mut todo_curr),
                Status::Done => list_up(&mut dones, &mut done_curr),
            },
            'j' => match status {
                Status::Todo => list_down(&mut todos, &mut todo_curr),
                Status::Done => list_down(&mut dones, &mut done_curr),
            },
            ' ' => match status {
                Status::Todo => list_transfer(&mut dones, &mut todos, &mut todo_curr),
                Status::Done => list_transfer(&mut todos, &mut dones, &mut done_curr),
            },
            'J' => {
                status = status.toggle();
            }
            _ => {
                // todos.push(format!("{}", key));
            }
        }
    }
    endwin();
}
