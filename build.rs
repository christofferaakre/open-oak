use std::fs::File;
use std::io::prelude::*;
// circle vertices
fn main() {
    let mut vertices_source =
        String::from("use crate::structs::Vertex;\nconst VERTICES: [Vertex; 360] = [\n");

    for i in 0..=359 {
        let t = i as f32 * std::f32::consts::PI / 180.0;
        let x = 0.5 * t.cos();
        let y = 0.5 * t.sin();
        vertices_source.push_str(
            format!(
                "    Vertex {{\n        position  : [{}, {}],\n        tex_coords: [{}, {}],\n    }},\n",
                x,
                y,
                x + 0.5,
                y + 0.5
            )
            .as_str(),
        )
    }

    vertices_source.push_str("];");

    let mut file = std::fs::File::create("src/circle_vertices.rs").unwrap();
    file.write_all(vertices_source.as_bytes()).unwrap();
}
