use glium::texture::SrgbTexture2d;
use glium::vertex::VertexBuffer;
use glium::Program;
use std::collections::HashMap;


use crate::structs::Vertex;

pub struct ResourceManager {
    pub programs: HashMap<String, Program>,
    pub textures: HashMap<String, SrgbTexture2d>,
    pub vertex_buffers: HashMap<String, VertexBuffer<Vertex>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            programs: HashMap::new(),
            textures: HashMap::new(),
            vertex_buffers: HashMap::new(),
        }
    }

    pub fn load_texture(
        display: &glium::Display,
        image_path: &str,
    ) -> glium::texture::SrgbTexture2d {
        let image = image::io::Reader::open(image_path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        let image_dimensions = image.dimensions();

        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        return texture;
    }

    pub fn get_program(&self, name: &String) -> Option<&Program> {
        self.programs.get(name)
    }

    pub fn get_texture(&self, name: &String) -> Option<&SrgbTexture2d> {
        self.textures.get(name)
    }

    pub fn get_vertex_buffer(&self, name: &String) -> Option<&VertexBuffer<Vertex>> {
        self.vertex_buffers.get(name)
    }

    pub fn add_program(&mut self, name: &String, program: Program) {
        self.programs.insert(name.to_string(), program);
    }

    pub fn add_texture(&mut self, name: &String, texture: SrgbTexture2d) {
        self.textures.insert(name.to_string(), texture);
    }

    pub fn add_vertex_buffer(&mut self, name: &String, vertex_buffer: VertexBuffer<Vertex>) {
        self.vertex_buffers.insert(name.to_string(), vertex_buffer);
    }
}
