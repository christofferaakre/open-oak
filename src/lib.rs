//! Game engine for 2D games based on OpenGL. Uses glium as
//! the openGL binding. This project is in very early stages.
//! # Features
//! * Rendering textured objects on the screen
//! * Handling window, mouse, and keyboard events
//! # Example
//! Below is a simple example that renders a textured block to the screen.
//!```rust
//!use glium::Surface;
//!use open_oak::events::handle_events;
//!use open_oak::init::{init, Game};
//!use open_oak::resource_manager::ResourceManager;
//!use open_oak::shapes::rect::Rectangle;
//!use open_oak::traits::{Renderable, Shaders};
//!use open_oak::{Rad, Rgba, Vector2, VirtualKeyCode};
//!
//!use std::collections::HashSet;
//!
//!struct Block {
//!    rect: Rectangle,
//!}
//!
//!impl Block {
//!    fn new(position: Vector2<f32>, size: Vector2<f32>, color: Rgba<f32>, texture: String) -> Block {
//!        let rect = Rectangle::new(position, size, Rad(0.0), color, texture);
//!        Block { rect }
//!    }
//!}
//!
//!fn main() {
//!    // init game and destructure
//!    let game = init();
//!
//!    // destructure fields off Game
//!    let Game {
//!        display,
//!        event_loop,
//!        mut resource_manager,
//!        ..
//!    } = game;
//!
//!    // load block texture
//!    let texture_name = String::from("block");
//!    let texture = ResourceManager::load_texture(&display, "src/bin/textures/block.png");
//!    resource_manager.add_texture(&texture_name, texture);
//!
//!    // define block
//!    let mut block = Block::new(
//!        Vector2::new(0.5, 0.5),
//!        Vector2::new(0.3, 0.3),
//!        image::Rgba::from([1.0, 0.0, 0.0, 1.0]),
//!        texture_name.clone(),
//!    );
//!
//!    // init rectangle
//!    Rectangle::init(&mut resource_manager, &display);
//!
//!    let mut pressed_keys: HashSet<VirtualKeyCode> = HashSet::new();
//!    // game loop
//!    event_loop.run(move |ev, _, control_flow| {
//!        // handle events, keyboard input, etc.
//!        let keyboard_input = handle_events(ev, &mut pressed_keys);
//!
//!        if let Some(keyboard_input) = keyboard_input {
//!            match keyboard_input.virtual_keycode.unwrap() {
//!                VirtualKeyCode::Escape => {
//!                    std::process::exit(0);
//!                }
//!                _ => {}
//!            }
//!        }
//!
//!        let mut frame = display.draw();
//!        frame.clear_color(0.2, 0.3, 0.3, 1.0);
//!
//!        // DRAW START
//!        block.rect.draw(&mut frame, &resource_manager).unwrap();
//!
//!        frame.finish().unwrap();
//!        // DRAW END
//!    });
//!}
//!```

#[macro_use]
extern crate glium;
extern crate doc_comment;

pub mod events;

pub mod init;
pub mod macros;
pub mod resource_manager;
pub mod shapes;
pub mod structs;
pub mod traits;

pub use cgmath::{Rad, Vector2, Vector3, VectorSpace};
pub use glium::glutin;
pub use glium::glutin::event::VirtualKeyCode;
pub use glium::Surface;
pub use image::Rgba;
pub use std::time::Instant;
