use crate::fs::Entity;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::StatefulWidget,
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection, ListState},
};

#[derive(Debug)]
pub struct FileTree<'a> {
    children: &'a Vec<Entity>,
}

impl FileTree<'_> {
    pub fn new(children: &Vec<Entity>) -> FileTree {
        FileTree { children }
    }
}

impl StatefulWidget for FileTree<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let list = List::new(self.children.iter().map(|entity| entity.name.to_string()))
            .block(Block::bordered().title("File Tree"))
            .highlight_style(Style::default().bold().on_dark_gray())
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        list.render(area, buf, state);
    }
}
