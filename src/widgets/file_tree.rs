use std::cmp::Ordering;

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
    children: &'a mut Vec<Entity>,
}

impl FileTree<'_> {
    pub fn new(children: &mut Vec<Entity>) -> FileTree {
        FileTree { children }
    }
}

impl StatefulWidget for FileTree<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        self.children.sort_by(|a, b| {
            if a.is_dir && b.is_dir {
                a.name.cmp(&b.name)
            } else if a.is_dir && !b.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let entities = self
            .children
            .iter()
            .filter_map(|entity| match !entity.is_dir {
                true => Some(entity.name.to_string()),
                false => None,
            });

        let list = List::new(entities)
            .block(Block::bordered().title("File Tree"))
            .highlight_style(Style::default().on_dark_gray())
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        list.render(area, buf, state);
    }
}
