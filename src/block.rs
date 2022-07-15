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
}

impl Renderable for Block {
    fn get_name() -> String {
        String::from("block")
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn size(&self) -> f32 {
        self.size
    }

    fn position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

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
