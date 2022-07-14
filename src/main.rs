#[macro_use]
extern crate glium;

use std::io::Cursor;

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

    // load model
    let shape = glium::vertex::VertexBuffer::new(&display, &block::VERTICES).unwrap();

    // load texture
    let image = image::load(
        Cursor::new(&include_bytes!("../textures/block.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();

    let image_dimensions = image.dimensions();

    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);
        // DRAW START

        let uniforms = uniform! { tex: &texture };

        frame
            .draw(
                &shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &block_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}
