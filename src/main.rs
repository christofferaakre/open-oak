#[macro_use]
extern crate glium;

use glium::glutin;

use glium::Surface;

mod events;
use events::*;

mod block;

fn main() {
    // Initialise display
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("glutin")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0f32, 600.0f32));
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // compile shader program
    let block_vertex_src = std::fs::read_to_string("shaders/block.vs").unwrap();

    let block_fragment_src = std::fs::read_to_string("shaders/block.fs").unwrap();

    let block_program = glium::Program::from_source(
        &display,
        block_vertex_src.as_str(),
        block_fragment_src.as_str(),
        None,
    )
    .unwrap();

    let shape = glium::vertex::VertexBuffer::new(&display, &block::VERTICES).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);
        // DRAW START

        frame
            .draw(
                &shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &block_program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}
