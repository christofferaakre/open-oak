//! Module containing various utilities for initialising a game.

use glium::glutin;

use crate::resource_manager::ResourceManager;

/// Struct containing other useful structs for managing a game.
pub struct Game<'a> {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
    pub wb: glutin::window::WindowBuilder,
    pub cb: glutin::ContextBuilder<'a, glutin::NotCurrent>,
    pub resource_manager: ResourceManager,
}

/// The main function provided by this module. Initialises a window
/// and resource manager and returns a Game struct.
pub fn init<'a>() -> Game<'a> {
    // Initialise display
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("glutin")
        .with_inner_size(glutin::dpi::PhysicalSize::new(800.0f32, 600.0f32));
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb.clone(), cb.clone(), &event_loop)
        .expect("Failed to initialise display");

    let resource_manager = ResourceManager::new();

    Game {
        event_loop,
        display,
        wb,
        cb,
        resource_manager,
    }
}
