use std::io::Result;
use std::rc::Rc;

use file_tree::FileTree;
use navigator::Navigator;

mod file_tree;
mod fs;
mod navigator;
mod tui;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    terminal.clear()?;
    let navigator = Rc::new(Navigator::new());
    let app_result = FileTree::new(navigator).run(&mut terminal);
    tui::restore()?;
    app_result
}
