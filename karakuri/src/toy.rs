// TODO: Delete this
use crate::math::Vector2;
use crate::utils::Color;

pub struct Toy {
    pub position: Vector2,
    pub size: Vector2,
    pub velocity: Vector2,
    pub color: Color,
}

impl Toy {
    pub fn new(position: Vector2, size: Vector2, velocity: Vector2, color: Color) -> Self {
        Self {
            position,
            size,
            velocity,
            color,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.position.add(&self.velocity.to_scaled(delta_time));
    }
}
