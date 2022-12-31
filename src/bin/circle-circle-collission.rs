use glium::Surface;

use open_oak::events::handle_events;
use open_oak::init::{init, Game};
use open_oak::resource_manager::ResourceManager;
use open_oak::shapes::circle::{Circle, CircleCollider};
use open_oak::traits::{Renderable, Shaders};

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

    let texture_name = String::from("block");
    let texture = ResourceManager::load_texture(&display, "src/bin/textures/block.png");
    resource_manager.add_texture(&texture_name, texture);

    let mut circle1 = Circle::new(
        Vector2::new(0.5, 0.5),
        0.1,
        image::Rgba([1.0, 0.0, 0.0, 1.0]),
        texture_name.clone(),
    );

    let mut collider1 = CircleCollider::new(circle1.radius, circle1.position);

    let circle2 = Circle::new(
        Vector2::new(0.5, 0.5),
        0.1,
        image::Rgba([0.0, 0.0, 1.0, 1.0]),
        texture_name.clone(),
    );

    let mut collider2 = CircleCollider::new(circle2.radius, circle2.position);

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
                    circle1.position += Vector2::new(0.0, -1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::A => {
                    circle1.position += Vector2::new(-1.0, 0.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::S => {
                    circle1.position += Vector2::new(0.0, 1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::D => {
                    circle1.position += Vector2::new(1.0, 0.0) * dt.as_secs_f32() * v;
                }
                _ => {}
            }
        }

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        collider1.center = circle1.position;
        collider2.center = circle2.position;

        let distance = (collider1.center - collider2.center).magnitude();

        circle1.draw(&mut frame, &resource_manager).unwrap();
        circle2.draw(&mut frame, &resource_manager).unwrap();

        // for circle in [&circle1, &circle2] {
        //     let mut center = Circle::new(circle.position, 0.1, image::Rgba([0.0, 1.0, 0.0, 1.0]));
        //     center.set_texture(texture_name.clone());
        //     center.draw(&mut frame, &resource_manager).unwrap();
        // }

        let collission = collider1.is_colliding_with(&collider2);
        let collission2 = collider2.is_colliding_with(&collider1);
        println!(
            "{} {}, positions: ({}, {}) ({}, {}). distance: {}, radius1: {}, radius2: {}",
            collission,
            collission2,
            circle1.position.x,
            circle1.position.y,
            circle2.position.x,
            circle2.position.y,
            distance,
            circle1.radius,
            circle2.radius
        );

        frame.finish().unwrap();
    });
}
