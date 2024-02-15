use karakuri::math::Vector2;
use karakuri::utils::Color;

fn main() {
    let v = Vector2::zero();
    let c = Color(0, 200, 200, 255);

    println!("{:#?}", v);
    println!("{:#?}", c);
}
