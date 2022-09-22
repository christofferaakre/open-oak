//! Module defining a rectangle.

use cgmath::Vector2;
use uuid::Uuid;

use glium::vertex::VertexBuffer;

use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use crate::traits::{Name, Renderable, Shaders, Texture, Vertices};

use glium::Surface;

use crate::impl_texture;

use cgmath::Rad;

/// Struct representing a Rectangle. Implements the `Renderable`
/// trait, so it can be rendered to the screen
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub rotation: Rad<f32>,
    pub id: uuid::Uuid,
    pub texture_name: String,
    pub color: image::Rgba<f32>,
}

impl_texture!(
    Rectangle,
    "rectangle",
    "../shaders/rectangle.vs",
    "../shaders/rectangle.fs"
);

impl Rectangle {
    /// Returns a new Rectangle with no texture.
    pub fn new(
        position: Vector2<f32>,
        size: Vector2<f32>,
        rotation: Rad<f32>,
        color: image::Rgba<f32>,
    ) -> Self {
        let block = Rectangle {
            position,
            size,
            id: Uuid::new_v4(),
            texture_name: Default::default(),
            color,
            rotation,
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
            -1.0 + self.position.x * 2.0,
            1.0 - self.position.y * 2.0,
            0.0,
        ));
        // let translation = cgmath::Matrix4::from_translation(cgmath::vec3(
        //     2.0 * (self.size.x / 2.0 - 0.5 + self.position.x),
        //     2.0 * (0.5 - self.size.y / 2.0 - self.position.y),
        //     0.0,
        // ));

        // rotation
        let rotation = cgmath::Matrix4::from_angle_z(self.rotation);

        let model = translation * rotation * scale;
        let model: [[f32; 4]; 4] = model.into();

        let texture = resource_manager
            .get_texture(&self.texture_name)
            .unwrap_or_else(|| panic!("Failed to get texture with name {}", &self.texture_name));

        let color = self.color.0;

        let uniforms = uniform! { tex: texture,
        model: model, color: color};

        let name = Self::get_name();
        let shape = resource_manager
            .get_vertex_buffer(&name)
            .unwrap_or_else(|| panic!("Could not retrieve vertex buffer with name {}", &name));
        let program = resource_manager
            .get_program(&name)
            .unwrap_or_else(|| panic!("Failed to get shader program with name {}", &name));

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

/// Struct representing a box collider (Axis Aligned Bounded Box)
pub struct RectangleCollider {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

#[derive(Debug)]
pub struct Edges {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl RectangleCollider {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        RectangleCollider { position, size }
    }

    pub fn edges(&self) -> Edges {
        let pos = self.position;
        let size = self.size;
        Edges {
            left: pos.x - size.x / 2.0,
            right: pos.x + size.x / 2.0,
            bottom: pos.y + size.y / 2.0,
            top: pos.y - size.y / 2.0,
        }
    }

    pub fn is_colliding_with_rect(&self, other: &RectangleCollider) -> bool {
        // check horizontal overlap
        let edges = self.edges();
        let other_edges = other.edges();
        let x_overlap = (edges.left <= other_edges.right && edges.right >= other_edges.right)
            || (edges.right >= other_edges.left && edges.left <= other_edges.left);
        let y_overlap = (edges.bottom >= other_edges.top && edges.top <= other_edges.top)
            || (edges.top <= other_edges.bottom && edges.bottom >= other_edges.bottom);

        return x_overlap && y_overlap;
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
