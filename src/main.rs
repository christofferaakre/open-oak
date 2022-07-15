#[macro_use]
extern crate glium;

use std::io::Cursor;

use glium::glutin;

use glium::Surface;

use uuid::Uuid;

mod events;
use events::*;

mod block;
mod init;
mod resrouce_manager;
mod structs;
mod traits;

use traits::Renderable;

use resrouce_manager::ResourceManager;

fn main() {
    let game = init::init();
    let display = game.display;
    let event_loop = game.event_loop;
    let mut resource_manager = game.resource_manager;

    block::Block::init(&display, &mut resource_manager);
    let mut block = block::Block::new(
        &display,
        &mut resource_manager,
        cgmath::Vector2::new(0.0, 0.0),
        1.0 / 8.0,
    );

    // load block texture
    let texture_name = String::from("block");

    let texture = ResourceManager::load_texture(&display, "textures/block.png");
    resource_manager.add_texture(&texture_name, texture);
    block.set_texture(texture_name);

    // game loop
    event_loop.run(move |ev, _, control_flow| {
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        block.draw(&mut frame, &resource_manager).unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}
