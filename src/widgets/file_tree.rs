use std::cmp::Ordering;

use crate::navigator::Navigator;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::StatefulWidget,
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection, ListState},
};

#[derive(Debug)]
pub struct FileTree<'a> {
    navigator: &'a mut Navigator,
}

impl FileTree<'_> {
    pub fn new(navigator: &mut Navigator) -> FileTree {
        FileTree { navigator }
    }
}

impl StatefulWidget for FileTree<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let entities = self.navigator.entities();
        entities.sort_by(|a, b| {
            if a.is_dir && b.is_dir {
                a.file_name.to_string().cmp(&b.file_name.to_string())
            } else if a.is_dir && !b.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let entities = entities.iter().map(|entity| entity.file_name.to_string());

        let list = List::new(entities)
            .block(Block::bordered().title("Dir Tree"))
            .highlight_style(Style::default().on_dark_gray())
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        list.render(area, buf, state);
    }
}
