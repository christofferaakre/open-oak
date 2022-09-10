use glium::Surface;

use open_oak::events::handle_events;
use open_oak::init::{init, Game};
use open_oak::rectangle::{Rectangle, RectangleCollider};
use open_oak::resource_manager::ResourceManager;
use open_oak::traits::{Renderable, Shaders, Texture};

use glium::glutin::event::VirtualKeyCode;

use cgmath::Vector2;

use std::collections::HashSet;

use std::time::Instant;

fn main() {
    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = init();

    Rectangle::init(&mut resource_manager, &display);

    let texture_name = String::from("block");
    let texture = ResourceManager::load_texture(&display, "src/bin/textures/block.png");
    resource_manager.add_texture(&texture_name, texture);

    let mut rect1 = Rectangle::new(
        Vector2::new(0.5, 0.5),
        Vector2::new(0.1, 0.1),
        image::Rgba([1.0, 0.0, 0.0, 1.0]),
    );

    let mut collider1 = RectangleCollider::new(rect1.position, rect1.size);

    let mut rect2 = Rectangle::new(
        Vector2::new(0.55, 0.55),
        Vector2::new(0.1, 0.1),
        image::Rgba([1.0, 0.0, 0.0, 1.0]),
    );

    let mut collider2 = RectangleCollider::new(rect2.position, rect2.size);

    rect1.set_texture(texture_name.clone());
    rect2.set_texture(texture_name.clone());

    let mut pressed_keys = HashSet::new();

    let v = 0.1;

    let mut last_frame = Instant::now();
    event_loop.run(move |ev, _, control_flow| {
        let dt = last_frame.elapsed();
        last_frame += dt;

        handle_events(ev, &mut pressed_keys);

        for key in pressed_keys.iter() {
            match key {
                &VirtualKeyCode::W => {
                    rect1.position += Vector2::new(0.0, -1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::A => {
                    rect1.position += Vector2::new(-1.0, 0.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::S => {
                    rect1.position += Vector2::new(0.0, 1.0) * dt.as_secs_f32() * v;
                }
                &VirtualKeyCode::D => {
                    rect1.position += Vector2::new(1.0, 0.0) * dt.as_secs_f32() * v;
                }
                _ => {}
            }
        }

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        collider1.position = rect1.position;
        collider2.position = rect2.position;

        rect1.draw(&mut frame, &resource_manager).unwrap();
        rect2.draw(&mut frame, &resource_manager).unwrap();

        let collission = collider1.is_colliding_with_rect(&collider2);
        let collission2 = collider2.is_colliding_with_rect(&collider1);
        println!("{} {}", collission, collission2);

        frame.finish().unwrap();
    });
}
