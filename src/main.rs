#[macro_use]
extern crate glium;

use glium::glutin;

use glium::Surface;

mod events;
use events::*;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("glutin")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0f32, 600.0f32));
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.finish().unwrap();
    });
}
