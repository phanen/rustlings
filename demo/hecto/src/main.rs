mod document;
mod editor;
mod row;
mod terminal;

// port to root
pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
