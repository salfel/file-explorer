use crate::fs::{get_current_entity, Entity};
use std::io::{self, prelude::*};

pub struct Navigator {
    current: Entity,
}

impl Navigator {
    pub fn new() -> Navigator {
        Navigator {
            current: get_current_entity(),
        }
    }

    fn update_dir(&mut self) {
        let stdin = io::stdin();
        let mut path = String::new();
        let line = stdin.lock().read_line(&mut path);

        if line.is_err() {
            return;
        }

        path.remove(path.len() - 1);

        if path == ".." {
            match self.current.parent().take().map(|parent| *parent) {
                Some(current) => self.current = current,
                None => (),
            }

            return;
        }

        for (idx, entity) in self.current.children.iter().enumerate() {
            if entity.is_dir && entity.name == path {
                let mut entity = self.current.children.remove(idx);
                entity.populate_children();
                self.current = entity;
                break;
            }
        }
    }
}
