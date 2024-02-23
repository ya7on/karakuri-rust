use crate::math::Vector2;

pub struct Transform {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Transform {
    pub fn new(position: Option<Vector2>, scale: Option<Vector2>, rotation: Option<f64>) -> Self {
        Self {
            position: position.unwrap_or(Vector2::zero()),
            scale: scale.unwrap_or(Vector2::new(1., 1.)),
            rotation: rotation.unwrap_or(0.),
        }
    }
}
