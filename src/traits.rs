use crate::structs::Vertex;
use crate::ResourceManager;
use glium::vertex::VertexBuffer;

use glium::Surface;

use uuid::Uuid;

use cgmath::Vector2;

use glium::texture::SrgbTexture2d;

pub trait Renderable {
    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex>;
    fn get_program(display: &glium::Display) -> glium::Program;

    fn get_name() -> String;

    fn init(display: &glium::Display, resource_manager: &mut ResourceManager) {
        let name = Self::get_name();
        let program = Self::get_program(display);
        resource_manager.add_program(&name, program);

        let vertex_buffer = Self::get_vertex_buffer(display);
        resource_manager.add_vertex_buffer(&name, vertex_buffer);
    }

    fn id(&self) -> Uuid;
    fn texture_name(&self) -> String;
    fn set_texture(&mut self, name: String);
    fn size(&self) -> Vector2<f32>;
    fn position(&self) -> Vector2<f32>;
    fn color(&self) -> image::Rgba<f32>;
    fn draw(
        &self,
        frame: &mut glium::Frame,
        resource_manager: &ResourceManager,
    ) -> Result<(), glium::DrawError> {
        let scale = cgmath::Matrix4::from_scale(self.size().x * 2.0);
        let translation = cgmath::Matrix4::from_translation(cgmath::vec3(
            2.0 * (self.size().x / 2.0 - 0.5 + self.position().x),
            2.0 * (0.5 - self.size().y / 2.0 - self.position().y),
            0.0,
        ));

        let model = translation * scale;
        let model: [[f32; 4]; 4] = model.into();

        let texture = resource_manager.get_texture(&self.texture_name()).unwrap();

        let color = self.color().0;

        let uniforms = uniform! { tex: texture,
        model: model, color: color};

        let name = Self::get_name();
        let shape = resource_manager.get_vertex_buffer(&name).unwrap();
        let program = resource_manager.get_program(&name).unwrap();

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
