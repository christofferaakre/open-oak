use cgmath::Vector2;
use uuid::Uuid;

use std::io::Cursor;

pub struct Block {
    pub position: Vector2<f32>,
    pub size: f32,
    pub id: uuid::Uuid,
}

use glium::vertex::VertexBuffer;
use glium::Surface;

use crate::resrouce_manager::ResourceManager;
use crate::structs::Vertex;
use crate::traits::Renderable;

impl Block {
    pub fn new(
        display: &glium::Display,
        resource_manager: &mut ResourceManager,
        position: Vector2<f32>,
        size: f32,
    ) -> Self {
        let image = image::load(
            Cursor::new(&include_bytes!("../textures/block.png")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();

        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        let id = Uuid::new_v4();
        resource_manager.add_texture(id, texture);

        Block { position, size, id }
    }

    pub fn init(display: &glium::Display, resource_manager: &mut ResourceManager) {
        let name = Self::get_name();
        let program = Self::get_program(display);
        resource_manager.add_program(&name, program);

        let vertex_buffer = Self::get_vertex_buffer(display);
        resource_manager.add_vertex_buffer(&name, vertex_buffer);
    }

    fn get_name() -> String {
        String::from("block")
    }
}

impl Renderable for Block {
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, &VERTICES).unwrap()
    }

    fn get_program(display: &glium::Display) -> glium::Program {
        let vertex_src = std::fs::read_to_string("shaders/block.vs").unwrap();

        let fragment_src = std::fs::read_to_string("shaders/block.fs").unwrap();

        let program =
            glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
                .unwrap();

        return program;
    }

    fn draw(&self, frame: &mut glium::Frame, resource_manager: &ResourceManager) {
        let block_pos: [f32; 2] = self.position.into();

        let scale = cgmath::Matrix4::from_scale(self.size * 2.0);
        let translation = cgmath::Matrix4::from_translation(cgmath::vec3(
            2.0 * (self.size / 2.0 - 0.5 + self.position.x),
            2.0 * (0.5 - self.size / 2.0 - self.position.y),
            0.0,
        ));

        let model = translation * scale;
        let model: [[f32; 4]; 4] = model.into();

        let texture = resource_manager.get_texture(self.id).unwrap();
        let uniforms = uniform! { tex: texture,
        model: model};

        let name = Self::get_name();
        let shape = resource_manager.get_vertex_buffer(&name).unwrap();
        let program = resource_manager.get_program(&name).unwrap();

        frame
            .draw(
                shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

pub const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
];
