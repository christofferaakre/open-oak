//! Module storing various structs.

/// Struct representing a vertex that will be stored in
/// a vertex buffer.
/// This struct is passed to the `glium::macros::implement_vertex` macro as follows:
/// `implement_vertex!(Vertex, position, tex_cords);
#[derive(Copy, Clone)]
pub struct Vertex {
    /// Coordinates of the vertices. Should be in the range [-1, 1]
    pub position: [f32; 2],
    /// Coordinates to sample texture colors from. Should be in the range [0, 1]
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
