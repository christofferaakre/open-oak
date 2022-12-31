use glium::Surface;

use open_oak::events::handle_events;
use open_oak::init::{init, Game};
use open_oak::resource_manager::ResourceManager;
use open_oak::shapes::circle::circle::{Circle, CircleCollider};
use open_oak::shapes::rect::rect::{Rectangle, RectangleCollider};
use open_oak::traits::{Renderable, Shaders, Texture};

use glium::glutin::event::VirtualKeyCode;

use cgmath::InnerSpace;
use cgmath::Vector2;

use std::collections::HashSet;

use std::time::Instant;

use open_oak::shapes::collission::Collide;

fn main() {
    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = init();

    Circle::init(&mut resource_manager, &display);
    Rectangle::init(&mut resource_manager, &display);

    let texture_name = String::from("block");
    let texture = ResourceManager::load_texture(&display, "src/bin/textures/block.png");
    resource_manager.add_texture(&texture_name, texture);

    let mut circle = Circle::new(
        Vector2::new(0.5, 0.5),
        0.15,
        image::Rgba([1.0, 0.0, 0.0, 1.0]),
        texture_name.clone(),
    );

    let mut circle_collider = CircleCollider::new(circle.radius, circle.position);

    let mut rect = Rectangle::new(
        Vector2::new(0.5, 0.5),
        Vector2::new(0.1, 0.1),
        cgmath::Rad(0.0),
        image::Rgba([0.0, 0.0, 1.0, 1.0]),
        texture_name.clone(),
    );

    let mut rect_collider = RectangleCollider::new(rect.position, rect.size, rect.rotation);

    let mut pressed_keys = HashSet::new();

    let v = 0.1;

    let mut last_frame = Instant::now();
    event_loop.run(move |ev, _, _control_flow| {
        let dt = last_frame.elapsed();
        last_frame += dt;

        handle_events(ev, &mut pressed_keys);

        for key in pressed_keys.iter() {
            match key {
                &VirtualKeyCode::W => {
                    circle.position += Vector2::new(0.0, -1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::A => {
                    circle.position += Vector2::new(-1.0, 0.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::S => {
                    circle.position += Vector2::new(0.0, 1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::D => {
                    circle.position += Vector2::new(1.0, 0.0) * dt.as_secs_f32() * v;
                }
                _ => {}
            }
        }

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        circle_collider.center = circle.position;
        rect_collider.position = rect.position;

        circle.draw(&mut frame, &resource_manager).unwrap();
        rect.draw(&mut frame, &resource_manager).unwrap();

        let collision = rect_collider.is_colliding_with(&circle_collider);
        if collision {
            rect.color = image::Rgba([0.0, 1.0, 0.0, 1.0]);
            circle.color = image::Rgba([0.0, 1.0, 0.0, 1.0]);
            println!("Collission");
        } else {
            rect.color = image::Rgba([1.0, 0.0, 0.0, 1.0]);
            circle.color = image::Rgba([0.0, 0.0, 1.0, 1.0]);
        }

        frame.finish().unwrap();
    });
}
