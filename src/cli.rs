use crate::fs::{get_current_entity, Entity};
use std::io::{self, prelude::*};

pub struct Writer {
    current_dir: Entity,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            current_dir: get_current_entity(),
        }
    }

    pub fn start(&mut self) {
        self.display_directories();

        loop {
            let entity = self.get_directory();
            if let Some(entity) = entity {
                self.current_dir = entity;

                self.display_directories();
            } else {
                println!("couldn't find directory");
            }
        }
    }

    fn display_directories(&mut self) {
        if self.current_dir.children.is_empty() {
            println!("there are no entries in this directory");
        }

        for entity in self.current_dir.children.iter() {
            println!("{}", entity);
        }
    }

    fn get_directory(&mut self) -> Option<Entity> {
        let stdin = io::stdin();
        let mut path = String::new();
        let line = stdin.lock().read_line(&mut path);

        if line.is_err() {
            return None;
        }

        path.remove(path.len() - 1);

        if path == ".." {
            return self.current_dir.parent().take().map(|parent| *parent);
        }

        for (idx, entity) in self.current_dir.children.iter().enumerate() {
            if entity.is_dir && entity.name == path {
                let mut entity = self.current_dir.children.remove(idx);
                entity.populate_children();
                return Some(entity);
            }
        }

        None
    }
}
