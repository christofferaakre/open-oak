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

use block::Block;
use traits::Renderable;

use resrouce_manager::ResourceManager;

fn main() {
    // init game and destructure
    let game = init::init();

    // destructure fields off Game
    let init::Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = game;

    // initialize blocks
    Block::init(&display, &mut resource_manager);

    // load block texture
    let texture_name = String::from("block");
    let texture = ResourceManager::load_texture(&display, "textures/block.png");
    resource_manager.add_texture(&texture_name, texture);

    // define block

    // set block texture

    let mut blocks: Vec<Block> = vec![];

    for i in 0..8 {
        let mut block = Block::new(
            &display,
            &mut resource_manager,
            cgmath::Vector2::new(i as f32 / 8.0, 0.0),
            1.0 / 8.0,
        );
        block.set_texture(texture_name.clone());
        blocks.push(block);
    }

    // game loop
    event_loop.run(move |ev, _, control_flow| {
        // handle events, keyboard input, etc.
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        for block in blocks.iter() {
            block.draw(&mut frame, &resource_manager).unwrap();
        }
        frame.finish().unwrap();
        // DRAW END
    });
}
