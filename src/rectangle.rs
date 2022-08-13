use cgmath::Vector2;
use std::io::Cursor;
use uuid::Uuid;

use glium::texture::SrgbTexture2d;
use glium::vertex::VertexBuffer;

use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use crate::traits::{Name, Renderable, Shaders, Texture, Vertices};

use glium::Surface;

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub id: uuid::Uuid,
    pub texture_name: String,
    pub color: image::Rgba<f32>,
}

impl Vertices for Rectangle {
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, &VERTICES).unwrap()
    }
}

impl Name for Rectangle {
    fn get_name() -> String {
        String::from("rectangle")
    }
}

impl Texture for Rectangle {
    fn init(resource_manager: &mut ResourceManager, display: &glium::Display) {
        let vertex_buffer = Rectangle::get_vertex_buffer(&display);
        resource_manager.add_vertex_buffer(&Rectangle::get_name(), vertex_buffer);

        resource_manager.add_program(&Rectangle::get_name(), Rectangle::get_program(&display));
    }

    fn set_texture(&mut self, texture_name: String) {
        self.texture_name = texture_name;
    }
}

impl Shaders for Rectangle {
    fn get_program(display: &glium::Display) -> glium::Program {
        let vertex_src = std::fs::read_to_string("shaders/block.vs").unwrap();
        let fragment_src = std::fs::read_to_string("shaders/block.fs").unwrap();

        let program =
            glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
                .unwrap();

        return program;
    }
}

impl Rectangle {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: image::Rgba<f32>) -> Self {
        let block = Rectangle {
            position,
            size,
            id: Uuid::new_v4(),
            texture_name: Default::default(),
            color,
        };

        return block;
    }
}

impl Renderable for Rectangle {
    fn draw(
        &self,
        frame: &mut glium::Frame,
        resource_manager: &ResourceManager,
    ) -> Result<(), glium::DrawError> {
        let scale =
            cgmath::Matrix4::from_nonuniform_scale(self.size.x * 2.0, self.size.y * 2.0, 1.0);
        let translation = cgmath::Matrix4::from_translation(cgmath::vec3(
            2.0 * (self.size.x / 2.0 - 0.5 + self.position.x),
            2.0 * (0.5 - self.size.y / 2.0 - self.position.y),
            0.0,
        ));

        let model = translation * scale;
        let model: [[f32; 4]; 4] = model.into();

        let texture = resource_manager.get_texture(&self.texture_name).unwrap();

        let color = self.color.0;

        let uniforms = uniform! { tex: texture,
        model: model, color: color};

        let name = Self::get_name();
        let shape = resource_manager
            .get_vertex_buffer(&name)
            .expect("Could not retrieve vertex buffer");
        let program = resource_manager.get_program(&name).unwrap();

        let params = glium::DrawParameters {
            // depth: glium::Depth {
            // test: glium::draw_parameters::DepthTest::IfEqual,
            // write: true,
            // ..Default::default()
            // },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        frame.draw(
            shape,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            program,
            &uniforms,
            &params,
        )
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
