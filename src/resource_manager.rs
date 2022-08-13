//! Module providing the ResourceManager struct for managing
//! resources such as shaders, textures, vertex buffers, etc.

use glium::texture::SrgbTexture2d;
use glium::vertex::VertexBuffer;
use glium::Program;
use std::collections::HashMap;

use crate::structs::Vertex;

/// The main struct provided by this module. Manages
/// shaders (shader programs), textures, and vertex buffers.
pub struct ResourceManager {
    pub programs: HashMap<String, Program>,
    pub textures: HashMap<String, SrgbTexture2d>,
    pub vertex_buffers: HashMap<String, VertexBuffer<Vertex>>,
}

impl ResourceManager {
    /// Returns a new empty ResourceManager.
    pub fn new() -> Self {
        ResourceManager {
            programs: HashMap::new(),
            textures: HashMap::new(),
            vertex_buffers: HashMap::new(),
        }
    }

    /// Loads a texture from the image path provided and returns it.
    /// # Panics
    /// * If the image path cannot be opened
    /// * If the image cannot be decoded
    /// * If the texture cannot be created
    pub fn load_texture(
        display: &glium::Display,
        image_path: &str,
    ) -> glium::texture::SrgbTexture2d {
        let image = image::io::Reader::open(image_path)
            .unwrap_or_else(|_| panic!("Failed to open file {}", image_path))
            .decode()
            .unwrap_or_else(|_| panic!("Failed to decode image {}", image_path))
            .to_rgba8();

        let image_dimensions = image.dimensions();

        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::SrgbTexture2d::new(display, image)
            .unwrap_or_else(|_| panic!("Failed to create texture from image {}", image_path));
        return texture;
    }

    /// Get a stored shader program from the resource manager.
    pub fn get_program(&self, name: &String) -> Option<&Program> {
        self.programs.get(name)
    }

    /// Get a stored texture from the resource manager.
    pub fn get_texture(&self, name: &String) -> Option<&SrgbTexture2d> {
        self.textures.get(name)
    }

    /// Get a stored vertex buffer from the resource manager.
    pub fn get_vertex_buffer(&self, name: &String) -> Option<&VertexBuffer<Vertex>> {
        self.vertex_buffers.get(name)
    }

    /// Store a shader program in the resource manager.
    pub fn add_program(&mut self, name: &String, program: Program) {
        self.programs.insert(name.to_string(), program);
    }

    /// Store a texture in the resource manager.
    pub fn add_texture(&mut self, name: &String, texture: SrgbTexture2d) {
        self.textures.insert(name.to_string(), texture);
    }

    /// Store a vertex buffer in the resource manager.
    pub fn add_vertex_buffer(&mut self, name: &String, vertex_buffer: VertexBuffer<Vertex>) {
        self.vertex_buffers.insert(name.to_string(), vertex_buffer);
    }
}
