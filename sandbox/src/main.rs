use karakuri::{
    math::Vector2,
    utils::{Color, Resolution},
    Engine,
};

fn main() {
    let mut engine = Engine::new(
        Some("Tree"),
        Some(Resolution::new(1920, 1080)),
        Some(Color::new(0, 30, 0, 255)),
        Some(60),
        Some(30),
    );

    engine.run();
}
