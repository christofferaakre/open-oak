use std::fs::File;
use std::io::prelude::*;
// circle vertices
fn main() {
    let n_vertices = 100;
    let mut vertices_source = format!(
        "use crate::structs::Vertex;\n/// Vertices for rendering a circle.\npub const VERTICES: [Vertex; {}] = [\n",
        n_vertices
    );

    for i in 0..n_vertices {
        let t = 2.0 * std::f32::consts::PI * i as f32 / n_vertices as f32;
        // let t = (i as f32) * std::f32::consts::PI / 180.0;
        let x = 2.0 * t.cos();
        let y = 2.0 * t.sin();
        vertices_source.push_str(
            format!(
                "    Vertex {{\n        position  : [{}f32, {}f32],\n        tex_coords: [{}f32, {}f32],\n    }},\n",
                x,
                y,
                x + 0.5,
                y + 0.5
            )
            .as_str(),
        )
    }

    vertices_source.push_str("];");

    let mut file = std::fs::File::create("src/shapes/circle/circle_vertices.rs").unwrap();
    file.write_all(vertices_source.as_bytes()).unwrap();
}
