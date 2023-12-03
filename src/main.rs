#![warn(clippy::all, clippy::pedantic, clippy::restriction)]

mod editor;
mod terminal;
mod document; 
mod row;
mod highlighting;

use editor::Editor;

pub use editor::Position;
pub use editor::SearchDirection;
pub use document::Document;
pub use row::Row;
pub use terminal::Terminal; 

fn main() {
    Editor::default().run();
}

