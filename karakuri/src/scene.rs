use crate::{
    components::{Name, RigidBody, Transform},
    entity::Entity,
};

use self::scene_objects::{ComponentsPayload, ComponentsResult};

pub mod scene_objects;

pub struct Scene {
    next_id: usize,
    entities: Vec<Entity>,
    names: Vec<Name>,
    transforms: Vec<Transform>,
    rigid_bodies: Vec<Option<RigidBody>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: Vec::new(),
            transforms: Vec::new(),
            names: Vec::new(),
            rigid_bodies: Vec::new(),
        }
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn get_components(&self, entity: Entity) -> ComponentsResult {
        ComponentsResult {
            name: &self.names[entity.id],
            transform: &self.transforms[entity.id],
            rigid_body: &self.rigid_bodies[entity.id],
        }
    }

    pub fn add_entity(&mut self, components: ComponentsPayload) {
        let entity = Entity::new(self.next_id);

        self.entities.push(entity);

        self.names.push(components.name);
        self.transforms.push(
            components
                .transform
                .unwrap_or(Transform::new(None, None, None)),
        );
        self.rigid_bodies.push(components.rigid_body);

        self.next_id += 1;
    }
}
