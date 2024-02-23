use karakuri::{
    components::{Name, Transform},
    math::Vector2,
    scene_objects::ComponentsPayload,
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

    engine.scene().add_entity(ComponentsPayload {
        name: Name(String::from("Sonic")),
        transform: Some(Transform::new(Some(Vector2::new(100., 100.)), None, None)),
        rigid_body: None,
    });

    engine.run();
}
