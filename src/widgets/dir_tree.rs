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
pub struct DirTree<'a> {
    navigator: &'a mut Navigator,
}

impl DirTree<'_> {
    pub fn new(navigator: &mut Navigator) -> DirTree {
        DirTree { navigator }
    }
}

impl StatefulWidget for DirTree<'_> {
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

        let entities = entities.iter().filter_map(|entity| match entity.is_dir {
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
