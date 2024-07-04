use renderer::Renderer;
use std::io::Result;

mod widgets {
    pub mod file_tree;
}

mod fs;
mod navigator;
mod renderer;
mod tui;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    terminal.clear()?;
    let app_result = Renderer::new().run(&mut terminal);
    tui::restore()?;
    app_result
}
