//! Module storing various structs.

/// Struct representing a vertex that will be stored in
/// a vertex buffer.
/// This struct is passed to the `glium::macros::implement_vertex` macro as follows:
/// `implement_vertex!(Vertex, position, tex_cords);
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
