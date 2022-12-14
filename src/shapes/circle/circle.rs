use cgmath::Vector2;
use uuid::Uuid;

use glium::vertex::VertexBuffer;

use crate::resource_manager::ResourceManager;
use crate::structs::Vertex;
use crate::traits::{Name, Renderable, Shaders, Texture, Vertices};

use glium::Surface;

use crate::impl_texture;

use super::circle_vertices::VERTICES;

/// Struct representing a Circle. Implements the `Renderable`
/// trait, so it can be rendered to the screen
#[derive(Clone, Debug)]
pub struct Circle {
    /// The position of the center of the circle. The origin is at (0,0)
    pub position: Vector2<f32>,
    pub radius: f32,
    pub id: uuid::Uuid,
    pub texture_name: String,
    pub color: image::Rgba<f32>,
}

impl_texture!(
    Circle,
    "circle",
    "../../../shaders/circle.vs",
    "../../../shaders/circle.fs"
);

impl Circle {
    /// Returns a new Circle with a texture.
    pub fn new(
        position: Vector2<f32>,
        radius: f32,
        color: crate::Rgba<f32>,
        texture: String,
    ) -> Self {
        Circle {
            position,
            radius,
            id: Uuid::new_v4(),
            texture_name: texture,
            color,
        }
    }
}

impl Renderable for Circle {
    fn draw(
        &self,
        frame: &mut glium::Frame,
        resource_manager: &ResourceManager,
    ) -> Result<(), glium::DrawError> {
        let size = self.radius * 1.0;
        let scale = cgmath::Matrix4::from_nonuniform_scale(size, size, 1.0);
        let translation = cgmath::Matrix4::from_translation(cgmath::Vector3::new(
            -1.0 + self.position.x * 2.0,
            1.0 - self.position.y * 2.0,
            0.0,
        ));

        let model = translation * scale;
        let model: [[f32; 4]; 4] = model.into();

        let texture = resource_manager
            .get_texture(&self.texture_name)
            .unwrap_or_else(|| panic!("Failed to get texture with name {}", &self.texture_name));

        let color = self.color.0;

        let uniforms = uniform! { tex: texture,
        model: model, color: color};
        //
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
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
            program,
            &uniforms,
            &params,
        )
    }
}

pub struct CircleCollider {
    pub radius: f32,
    pub center: Vector2<f32>,
}

impl CircleCollider {
    pub fn new(radius: f32, center: Vector2<f32>) -> Self {
        CircleCollider { radius, center }
    }
}
