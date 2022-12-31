//! Module containing useful macros.

/// Implements the `Name` trait for the given struct.
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

/// Implements the `Vertices` trait for the given struct.
#[macro_export]
macro_rules! impl_vertices {
    ($struct:ident) => {
        impl Vertices for $struct {
            fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex> {
                VertexBuffer::new(display, &VERTICES).expect("Failed to create vertex buffer")
            }
        }
    };
}

/// Implements the `Shaders` trait for the given struct.
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

            fn init(resource_manager: &mut ResourceManager, display: &glium::Display) {
                let vertex_buffer = $struct::get_vertex_buffer(&display);
                resource_manager.add_vertex_buffer(&$struct::get_name(), vertex_buffer);

                resource_manager.add_program(&$struct::get_name(), $struct::get_program(&display));
            }
        }
    };
}

/// Implements the `Texture` trait for the given struct. Requires the trait bounds
/// to be satisfied already.
#[macro_export]
macro_rules! impl_texture_trait {
    ($struct:ident) => {
        impl Texture for $struct {
            fn set_texture(&mut self, texture_name: String) {
                self.texture_name = texture_name;
            }
        }
    };
}

/// Implements the `Texture` trait and all of its trait bounds.
#[macro_export]
macro_rules! impl_texture {
    ($struct:ident, $name:literal, $vertex_source:literal, $fragment_source:literal) => {
        crate::impl_name!($struct, $name);
        crate::impl_vertices!($struct);
        crate::impl_shaders!($struct, $vertex_source, $fragment_source);
        crate::impl_texture_trait!($struct);
    };
}
