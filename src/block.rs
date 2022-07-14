#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-0.5, -0.5],
        tex_coords: [-0.5, -0.5],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [-0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5],
        tex_coords: [0.5, 0.5],
    },
];
