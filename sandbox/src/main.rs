use karakuri::{
    math::Vector2, toy::Toy, utils::{Color, Resolution}, Engine
};

fn main() {
    let mut engine = Engine::new(
        Some("Tree"),
        Some(Resolution::new(1920, 1080)),
        Some(Color::new(0, 30, 0, 255)),
        Some(60),
        Some(30),
    );

    engine.add_toy(
        Toy::new(
            Vector2::new(100., 100.),
            Vector2::new(100., 100.),
            Vector2::new(50., 50.),
            Color::new(255, 0, 0, 255)
        )
    );

    engine.run();
}
