use crate::structs::Vertex;
use crate::ResourceManager;
use glium::vertex::VertexBuffer;

pub trait Renderable {
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex>;
    fn get_program(display: &glium::Display) -> glium::Program;
    fn draw(&self, frame: &mut glium::Frame, resource_manager: &ResourceManager);
}
