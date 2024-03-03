use karakuri::entity::{EntityItem, EntityTrait, SpatialAddon};
use karakuri::{
    math::Vector2,
    utils::{Color, Resolution},
    Engine,
};

pub struct RedSquareEntity {}

impl EntityTrait for RedSquareEntity {
    fn on_start(&mut self) {}

    fn on_step(&mut self, _delta_time: f64) {}
}

fn main() {
    let mut engine = Engine::new(
        Some("Tree"),
        Some(Resolution::new(1920, 1080)),
        Some(Color::new(0, 30, 0, 255)),
        Some(60),
        Some(30),
    );

    engine.scene().add_entity(
        EntityItem::init(RedSquareEntity {}).register_addon(SpatialAddon {
            position: Vector2::new(100., 100.),
            size: Vector2::new(100., 100.),
            color: Color::new(255, 0, 0, 255),
        }),
    );

    engine.scene().add_entity(
        EntityItem::init(RedSquareEntity {}).register_addon(SpatialAddon {
            position: Vector2::new(200., 200.),
            size: Vector2::new(100., 100.),
            color: Color::new(0, 0, 255, 255),
        }),
    );

    engine.run();
}
