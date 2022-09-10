//! Module containing various traits.

use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use glium::vertex::VertexBuffer;

/// Structs implementing this trait can be drawn to the screen.
/// To draw them, call the `draw` method.
pub trait Renderable: Texture {
    /// Method to draw the object on the screen.
    fn draw(
        &self,
        frame: &mut glium::Frame,
        resource_manager: &ResourceManager,
    ) -> Result<(), glium::DrawError>;
}

/// Structs implementing this trait can have a texture set on them.
pub trait Texture: Shaders {
    /// Sets the object's texture. The texture needs to be registered in the
    /// resource manager before calling this method.
    fn set_texture(&mut self, texture_name: String);
}

/// Structs implementing this trait have a shader program.
pub trait Shaders: Name + Vertices {
    /// Get the shader program the object is using.
    fn get_program(display: &glium::Display) -> glium::Program;
    /// This method should be called before you try to do anything involving shaders or vertex
    /// buffers.
    /// Registers the object's vertex buffer and shader program with the resource manager
    fn init(resource_manager: &mut ResourceManager, display: &glium::Display);
}

/// Structs implementing this have a name. Similar to a class variable in
/// object oriented languages.
pub trait Name {
    /// Returns the name of the object.
    fn get_name() -> String;
}

/// Structs implementing this trait have a vertex buffer.
pub trait Vertices: Name {
    /// Gets the vertex buffer that containst the vertices used to render the object.
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex>;
}
