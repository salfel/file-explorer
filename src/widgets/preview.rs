use std::{borrow::Borrow, cmp::Ordering, fs, process};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, List, ListDirection, Paragraph, Widget},
};

use crate::entity::Entity;

pub enum Previewer<'a> {
    Dir(&'a mut Entity),
    TextFile(&'a mut Entity),
    Other(&'a mut Entity),
}

impl Previewer<'_> {
    pub fn new(entity: &mut Entity) -> Previewer {
        if entity.is_dir {
            entity.parse_relatives();
            Previewer::Dir(entity)
        } else if !matches!(entity.file_name.extension.borrow(), "jpg" | "png" | "jpeg") {
            Previewer::TextFile(entity)
        } else {
            Previewer::Other(entity)
        }
    }
}

impl Widget for Previewer<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self {
            Previewer::Dir(entity) => render_dir(entity, area, buf),
            Previewer::TextFile(entity) => render_text_file(entity, area, buf),
            _ => {}
        }
    }
}

fn render_dir(entity: &mut Entity, area: Rect, buf: &mut Buffer) {
    let entities = &mut entity.children;
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
        .block(
            Block::bordered()
                .title(" Dir Preview ")
                .title_alignment(Alignment::Center),
        )
        .direction(ListDirection::TopToBottom);

    list.render(area, buf);
}

fn render_text_file(entity: &Entity, area: Rect, buf: &mut Buffer) {
    let contents = fs::read_to_string(&entity.path).unwrap_or_default();
    let paragraph = Paragraph::new(contents).block(
        Block::bordered()
            .title(" File Preview ")
            .title_alignment(Alignment::Center),
    );

    paragraph.render(area, buf)
}
