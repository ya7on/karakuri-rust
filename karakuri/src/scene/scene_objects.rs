use crate::components::{Name, RigidBody, Transform};

pub struct ComponentsPayload {
    pub name: Name,
    pub transform: Option<Transform>,
    pub rigid_body: Option<RigidBody>,
}

pub struct ComponentsResult<'a> {
    pub name: &'a Name,
    pub transform: &'a Transform,
    pub rigid_body: &'a Option<RigidBody>,
}
