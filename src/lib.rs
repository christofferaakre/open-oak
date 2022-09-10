//! Game engine for 2D games based on OpenGL. Uses glium as
//! the openGL binding. This project is in very early stages.
//! # Features
//! * Rendering textured objects on the screen
//! * Handling window, mouse, and keyboard events
//! # Example
//! Below is a simple example that renders a textured block to the screen.
//! ```rust
//!
//! use glium::Surface;
//!
//! use open_oak::events::handle_events;
//! use open_oak::init::{init, Game};
//! use open_oak::rectangle::Rectangle;
//! use open_oak::resource_manager::ResourceManager;
//! use open_oak::traits::{Renderable, Shaders, Texture};
//!
//! use cgmath::Vector2;
//!
//! struct Block {
//!     rect: Rectangle,
//! }
//!
//! impl Block {
//!     fn new(position: Vector2<f32>, size: Vector2<f32>, color: imaopen_oak::Rgba<f32>) -> Block {
//!         let rect = Rectangle::new(position, size, color);
//!         Block { rect }
//!     }
//! }
//!
//! fn main() {
//!     // init game and destructure
//!     let game = init();
//!
//!     // destructure fields off Game
//!     let Game {
//!         display,
//!         event_loop,
//!         mut resource_manager,
//!         ..
//!     } = game;
//!
//!     // define block
//!     let mut block = Block::new(
//!         Vector2::new(0.5, 0.5),
//!         Vector2::new(0.3, 0.3),
//!         image::Rgba::from([1.0, 0.0, 0.0, 1.0]),
//!     );
//!
//!     // init rectangle
//!     Rectangle::init(&mut resource_manager, &display);
//!
//!     // load block texture
//!     let texture_name = String::from("block");
//!     let texture = ResourceManager::load_texture(&display, "textures/block.png");
//!     resource_manager.add_texture(&texture_name, texture);
//!
//!     // set block texture
//!     block.rect.set_texture(texture_name.clone());
//!
//!     // game loop
//!     event_loop.run(move |ev, _, control_flow| {
//!         // handle events, keyboard input, etc.
//!         handle_events(ev, control_flow);
//!
//!         let mut frame = display.draw();
//!         frame.clear_color(0.2, 0.3, 0.3, 1.0);
//!
//!         // DRAW START
//!         block.rect.draw(&mut frame, &resource_manager).unwrap();
//!
//!         frame.finish().unwrap();
//!         // DRAW END
//!     });
//! }
//! ```

#[macro_use]
extern crate glium;
extern crate doc_comment;

pub mod events;

// pub mod block;
pub mod circle;
pub mod circle_vertices;
pub mod init;
pub mod macros;
pub mod rectangle;
pub mod resource_manager;
pub mod structs;
pub mod traits;
