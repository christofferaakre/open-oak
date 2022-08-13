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

#[macro_export]
macro_rules! impl_name {
    ($struct:ident, $name:literal) => {
        impl Name for $struct {
            fn get_name() -> String {
                String::from($name)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vertices {
    ($struct:ident) => {
        impl Vertices for $struct {
            fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex> {
                VertexBuffer::new(display, &VERTICES).unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_shaders {
    ($struct:ident, $vertex_source:literal, $fragment_source:literal) => {
        impl Shaders for $struct {
            fn get_program(display: &glium::Display) -> glium::Program {
                let vertex_src = include_str!($vertex_source);
                let fragment_src = include_str!($fragment_source);

                let program = glium::Program::from_source(display, vertex_src, fragment_src, None)
                    .expect("Could not compile shader program");

                return program;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_texture_trait {
    ($struct:ident) => {
        impl Texture for $struct {
            fn init(resource_manager: &mut ResourceManager, display: &glium::Display) {
                let vertex_buffer = Rectangle::get_vertex_buffer(&display);
                resource_manager.add_vertex_buffer(&Rectangle::get_name(), vertex_buffer);

                resource_manager
                    .add_program(&Rectangle::get_name(), Rectangle::get_program(&display));
            }

            fn set_texture(&mut self, texture_name: String) {
                self.texture_name = texture_name;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_texture {
    ($struct:ident, $name:literal, $vertex_source:literal, $fragment_source:literal) => {
        crate::impl_name!($struct, $name);
        crate::impl_vertices!($struct);
        crate::impl_shaders!($struct, $vertex_source, $fragment_source);
        crate::impl_texture_trait!($struct);
    };
}
