use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use glium::vertex::VertexBuffer;

use glium::Surface;

use uuid::Uuid;

use cgmath::Vector2;

pub trait Renderable: Texture {
    fn draw(
        &self,
        frame: &mut glium::Frame,
        resource_manager: &ResourceManager,
    ) -> Result<(), glium::DrawError>;
}

pub trait Texture: Shaders {
    fn init(resource_manager: &mut ResourceManager, display: &glium::Display);
    fn set_texture(&mut self, texture_name: String);
}

pub trait Shaders: Name + Vertices {
    fn get_program(display: &glium::Display) -> glium::Program;
}

pub trait Name {
    fn get_name() -> String;
}

pub trait Vertices: Name {
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex>;
}
