use crate::fs::{get_current_entity, trim_path, Entity};

#[derive(Debug)]
pub struct Navigator {
    current: Entity,
}

impl Navigator {
    pub fn new() -> Navigator {
        let mut navigator = Navigator {
            current: get_current_entity(),
        };

        navigator.add_parent();

        navigator
    }

    pub fn update_dir(&mut self, idx: usize) {
        let mut entity = self.current.children.remove(idx);
        entity.populate_children();
        self.current = entity;
        self.add_parent();
    }

    fn add_parent(&mut self) {
        let (_, path) = trim_path(&self.current.path);

        let parent = Entity::new(String::from(".."), path, true, false);

        self.current.children.insert(0, parent);
    }

    pub fn entities(&mut self) -> &mut Vec<Entity> {
        &mut self.current.children
    }
}
