use glium::glutin;

use crate::resource_manager::ResourceManager;

pub struct Game<'a> {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
    pub wb: glutin::window::WindowBuilder,
    pub cb: glutin::ContextBuilder<'a, glutin::NotCurrent>,
    pub resource_manager: ResourceManager,
}

pub fn init<'a>() -> Game<'a> {
    // Initialise display
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("glutin")
        .with_inner_size(glutin::dpi::PhysicalSize::new(800.0f32, 600.0f32));
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb.clone(), cb.clone(), &event_loop).unwrap();

    let resource_manager = ResourceManager::new();

    Game {
        event_loop,
        display,
        wb,
        cb,
        resource_manager,
    }
}
