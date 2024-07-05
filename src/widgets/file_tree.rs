use std::{borrow::BorrowMut, cmp::Ordering};

use crate::{fs::Entity, navigator::Navigator};
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
                a.name.cmp(&b.name)
            } else if a.is_dir && !b.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let entities = entities.iter().map(|entity| {
            if entity.is_dir {
                format!("  {}", entity.name)
            } else {
                format!("  {}", entity.name)
            }
        });

        let list = List::new(entities)
            .block(Block::bordered().title("File Tree"))
            .highlight_style(Style::default().on_dark_gray())
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        list.render(area, buf, state);
    }
}
