use crate::entity::EntityItem;

pub mod scene_objects;

#[derive(Default)]
pub struct Scene {
    entities: Vec<EntityItem>,
}

impl Scene {
    pub fn entities(&mut self) -> &mut Vec<EntityItem> {
        &mut self.entities
    }

    pub fn add_entity(&mut self, entity: EntityItem) {
        self.entities.push(entity);
    }
}
