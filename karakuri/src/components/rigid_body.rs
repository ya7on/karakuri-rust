use crate::math::Vector2;

pub struct RigidBody {
    pub velocity: Vector2,
}

impl RigidBody {
    pub fn new(velocity: Option<Vector2>) -> Self {
        Self {
            velocity: velocity.unwrap_or(Vector2::zero()),
        }
    }
}
