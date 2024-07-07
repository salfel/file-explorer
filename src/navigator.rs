use crate::entity::{get_current_entity, Entity};

#[derive(Debug)]
pub struct Navigator {
    pub current: Entity,
}

impl Navigator {
    pub fn new() -> Navigator {
        Navigator {
            current: get_current_entity(),
        }
    }

    pub fn update_dir(&mut self, idx: usize) {
        let entity = self.current.children.remove(idx);
        self.current = entity;
        self.current.parse_relatives();
    }

    pub fn entities(&mut self) -> &mut Vec<Entity> {
        &mut self.current.children
    }
}
