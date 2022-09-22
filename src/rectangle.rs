//! Module defining a rectangle.

use cgmath::Vector2;
use uuid::Uuid;

use glium::vertex::VertexBuffer;

use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use crate::traits::{Name, Renderable, Shaders, Texture, Vertices};

use glium::Surface;

use crate::impl_texture;

use cgmath::InnerSpace;
use cgmath::Rad;

use std::cmp::Ordering;

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

    fn edges(&self) -> Edges {
        let pos = self.position;
        let size = self.size;
        let rotation = cgmath::Matrix2::from_angle(-self.rotation);
        Edges {
            top_left: pos + rotation * Vector2::new(-size.x / 2.0, -size.y / 2.0),
            top_right: pos + rotation * Vector2::new(size.x / 2.0, -size.y / 2.0),
            bottom_left: pos + rotation * Vector2::new(-size.x / 2.0, size.y / 2.0),
            bottom_right: pos + rotation * Vector2::new(size.x / 2.0, size.y / 2.0),
        }
    }

    pub fn draw_edges(&self, frame: &mut glium::Frame, resource_manager: &ResourceManager) {
        let edges = self.edges();

        for edge in edges.iter() {
            let mut rect = Rectangle::new(
                edge.clone(),
                Vector2::new(0.03, 0.03),
                Rad(0.0),
                image::Rgba([0.0, 1.0, 0.0, 1.0]),
            );

            rect.set_texture(String::from("block"));

            rect.draw(frame, resource_manager)
                .expect("Failed to draw edge");
        }
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
    pub rotation: Rad<f32>,
}

#[derive(Debug)]
pub struct Edges {
    top_left: Vector2<f32>,
    top_right: Vector2<f32>,
    bottom_left: Vector2<f32>,
    bottom_right: Vector2<f32>,
}

impl Edges {
    pub fn iter(&self) -> [Vector2<f32>; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
}

impl RectangleCollider {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, rotation: Rad<f32>) -> Self {
        RectangleCollider {
            position,
            size,
            rotation,
        }
    }

    pub fn edges(&self) -> Edges {
        let pos = self.position;
        let size = self.size;
        let rotation = cgmath::Matrix2::from_angle(-self.rotation);
        Edges {
            top_left: pos + rotation * Vector2::new(-size.x / 2.0, -size.y / 2.0),
            top_right: pos + rotation * Vector2::new(size.x / 2.0, -size.y / 2.0),
            bottom_left: pos + rotation * Vector2::new(-size.x / 2.0, size.y / 2.0),
            bottom_right: pos + rotation * Vector2::new(size.x / 2.0, size.y / 2.0),
        }
    }

    /// Uses the Separating Axis Theorem to check if the rectangle collider is colliding with another rectangle collider
    pub fn is_colliding_with_rect(&self, other: &RectangleCollider) -> bool {
        // https://www.gamedev.net/tutorials/_/technical/game-programming/2d-rotated-rectangle-collision-r2604/

        // Use Separating Axis Theorem
        let edges = self.edges();
        let other_edges = other.edges();

        // Calculate the 4 axes perpendicular to the edges, 2 for each rectangle
        let axis1 = edges.top_right - edges.top_left;
        let axis2 = edges.top_right - edges.bottom_right;
        let axis3 = other_edges.top_right - other_edges.top_left;
        let axis4 = other_edges.top_right - other_edges.bottom_right;
        let axes = [axis1, axis2, axis3, axis4];

        // project edges onto axes
        for axis in axes {
            let projected_edges = edges
                .iter()
                .map(|edge| edge.dot(axis) / (axis.dot(axis)) * axis);
            let projected_other_edges = other_edges
                .iter()
                .map(|edge| edge.dot(axis) / (axis.dot(axis)) * axis);

            let dot_products = edges.iter().map(|edge| edge.dot(axis));
            let min = dot_products
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let max = dot_products
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let other_dot_products = other_edges.iter().map(|edge| edge.dot(axis));
            let other_min = other_dot_products
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let other_max = other_dot_products
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            // check for overlap. If even one axis have no overlap,
            // then there is no collission
            if !(other_min <= max && other_max >= min) {
                return false;
            }
        }

        // If every axis had an overlap, then there is a collission
        return true;
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
