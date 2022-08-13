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
