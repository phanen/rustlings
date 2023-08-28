mod editor;
mod terminal;

use editor::Editor;
// port to root
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
