use glium::Surface;

use open_oak::shapes::circle::circle::CircleCollider;
use open_oak::shapes::rect::rect::RectangleCollider;

use open_oak::shapes::collission::Collide;

use cgmath::{Rad, Vector2};
use image::Rgba;

fn main() {
    let rect1 = RectangleCollider::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Rad(0.0));
    let rect2 = RectangleCollider::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Rad(0.0));

    let circle1 = CircleCollider::new(1.0, Vector2::new(0.0, 0.0));
    let circle2 = CircleCollider::new(1.0, Vector2::new(0.0, 0.0));

    rect1.is_colliding_with(&rect2);
    rect1.is_colliding_with(&circle1);
    circle1.is_colliding_with(&rect1);
    circle1.is_colliding_with(&circle2);
}
