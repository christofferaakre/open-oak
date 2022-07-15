use glium::texture::SrgbTexture2d;
use glium::vertex::VertexBuffer;
use glium::Program;
use std::collections::HashMap;
use uuid::Uuid;

use crate::structs::Vertex;

pub struct ResourceManager {
    pub programs: HashMap<String, Program>,
    pub textures: HashMap<uuid::Uuid, SrgbTexture2d>,
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

    pub fn get_program(&self, name: &String) -> Option<&Program> {
        self.programs.get(name)
    }

    pub fn get_texture(&self, id: Uuid) -> Option<&SrgbTexture2d> {
        self.textures.get(&id)
    }

    pub fn get_vertex_buffer(&self, name: &String) -> Option<&VertexBuffer<Vertex>> {
        self.vertex_buffers.get(name)
    }

    pub fn add_program(&mut self, name: &String, program: Program) {
        self.programs.insert(name.to_string(), program);
    }

    pub fn add_texture(&mut self, id: Uuid, texture: SrgbTexture2d) {
        self.textures.insert(id, texture);
    }

    pub fn add_vertex_buffer(&mut self, name: &String, vertex_buffer: VertexBuffer<Vertex>) {
        self.vertex_buffers.insert(name.to_string(), vertex_buffer);
    }
}
