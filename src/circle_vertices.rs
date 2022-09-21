use crate::structs::Vertex;
/// Vertices for rendering a circle.
pub const VERTICES: [Vertex; 100] = [
    Vertex {
        position  : [2f32, 0f32],
        tex_coords: [2.5f32, 0.5f32],
    },
    Vertex {
        position  : [1.9960535f32, 0.12558104f32],
        tex_coords: [2.4960535f32, 0.625581f32],
    },
    Vertex {
        position  : [1.9842294f32, 0.25066647f32],
        tex_coords: [2.4842296f32, 0.7506665f32],
    },
    Vertex {
        position  : [1.9645745f32, 0.37476262f32],
        tex_coords: [2.4645743f32, 0.87476265f32],
    },
    Vertex {
        position  : [1.9371663f32, 0.4973798f32],
        tex_coords: [2.4371662f32, 0.9973798f32],
    },
    Vertex {
        position  : [1.9021131f32, 0.618034f32],
        tex_coords: [2.402113f32, 1.118034f32],
    },
    Vertex {
        position  : [1.859553f32, 0.7362491f32],
        tex_coords: [2.3595529f32, 1.2362491f32],
    },
    Vertex {
        position  : [1.8096541f32, 0.8515586f32],
        tex_coords: [2.3096542f32, 1.3515587f32],
    },
    Vertex {
        position  : [1.7526133f32, 0.9635074f32],
        tex_coords: [2.2526133f32, 1.4635074f32],
    },
    Vertex {
        position  : [1.6886559f32, 1.0716536f32],
        tex_coords: [2.1886559f32, 1.5716536f32],
    },
    Vertex {
        position  : [1.618034f32, 1.1755705f32],
        tex_coords: [2.118034f32, 1.6755705f32],
    },
    Vertex {
        position  : [1.5410265f32, 1.2748481f32],
        tex_coords: [2.0410266f32, 1.7748481f32],
    },
    Vertex {
        position  : [1.4579372f32, 1.3690943f32],
        tex_coords: [1.9579372f32, 1.8690943f32],
    },
    Vertex {
        position  : [1.3690941f32, 1.4579374f32],
        tex_coords: [1.8690941f32, 1.9579374f32],
    },
    Vertex {
        position  : [1.2748479f32, 1.5410266f32],
        tex_coords: [1.7748479f32, 2.0410266f32],
    },
    Vertex {
        position  : [1.1755705f32, 1.618034f32],
        tex_coords: [1.6755705f32, 2.118034f32],
    },
    Vertex {
        position  : [1.0716535f32, 1.6886559f32],
        tex_coords: [1.5716535f32, 2.1886559f32],
    },
    Vertex {
        position  : [0.96350724f32, 1.7526134f32],
        tex_coords: [1.4635072f32, 2.2526135f32],
    },
    Vertex {
        position  : [0.8515586f32, 1.8096541f32],
        tex_coords: [1.3515587f32, 2.3096542f32],
    },
    Vertex {
        position  : [0.7362491f32, 1.859553f32],
        tex_coords: [1.2362491f32, 2.3595529f32],
    },
    Vertex {
        position  : [0.61803395f32, 1.9021131f32],
        tex_coords: [1.1180339f32, 2.402113f32],
    },
    Vertex {
        position  : [0.49737966f32, 1.9371663f32],
        tex_coords: [0.99737966f32, 2.4371662f32],
    },
    Vertex {
        position  : [0.37476248f32, 1.9645746f32],
        tex_coords: [0.8747625f32, 2.4645746f32],
    },
    Vertex {
        position  : [0.25066647f32, 1.9842294f32],
        tex_coords: [0.7506665f32, 2.4842296f32],
    },
    Vertex {
        position  : [0.125581f32, 1.9960535f32],
        tex_coords: [0.625581f32, 2.4960535f32],
    },
    Vertex {
        position  : [-0.00000008742278f32, 2f32],
        tex_coords: [0.4999999f32, 2.5f32],
    },
    Vertex {
        position  : [-0.12558118f32, 1.9960535f32],
        tex_coords: [0.37441882f32, 2.4960535f32],
    },
    Vertex {
        position  : [-0.25066665f32, 1.9842293f32],
        tex_coords: [0.24933335f32, 2.4842293f32],
    },
    Vertex {
        position  : [-0.37476286f32, 1.9645745f32],
        tex_coords: [0.12523714f32, 2.4645743f32],
    },
    Vertex {
        position  : [-0.49737984f32, 1.9371663f32],
        tex_coords: [0.0026201606f32, 2.4371662f32],
    },
    Vertex {
        position  : [-0.61803406f32, 1.902113f32],
        tex_coords: [-0.118034065f32, 2.402113f32],
    },
    Vertex {
        position  : [-0.73624927f32, 1.8595529f32],
        tex_coords: [-0.23624927f32, 2.3595529f32],
    },
    Vertex {
        position  : [-0.85155874f32, 1.809654f32],
        tex_coords: [-0.35155874f32, 2.309654f32],
    },
    Vertex {
        position  : [-0.9635076f32, 1.7526132f32],
        tex_coords: [-0.4635076f32, 2.252613f32],
    },
    Vertex {
        position  : [-1.0716538f32, 1.6886557f32],
        tex_coords: [-0.57165384f32, 2.1886559f32],
    },
    Vertex {
        position  : [-1.1755708f32, 1.6180338f32],
        tex_coords: [-0.67557085f32, 2.118034f32],
    },
    Vertex {
        position  : [-1.274848f32, 1.5410265f32],
        tex_coords: [-0.774848f32, 2.0410266f32],
    },
    Vertex {
        position  : [-1.3690943f32, 1.4579372f32],
        tex_coords: [-0.86909425f32, 1.9579372f32],
    },
    Vertex {
        position  : [-1.4579372f32, 1.3690941f32],
        tex_coords: [-0.95793724f32, 1.8690941f32],
    },
    Vertex {
        position  : [-1.5410266f32, 1.2748479f32],
        tex_coords: [-1.0410266f32, 1.7748479f32],
    },
    Vertex {
        position  : [-1.6180341f32, 1.1755704f32],
        tex_coords: [-1.1180341f32, 1.6755704f32],
    },
    Vertex {
        position  : [-1.688656f32, 1.0716534f32],
        tex_coords: [-1.188656f32, 1.5716534f32],
    },
    Vertex {
        position  : [-1.7526134f32, 0.9635071f32],
        tex_coords: [-1.2526134f32, 1.4635072f32],
    },
    Vertex {
        position  : [-1.8096542f32, 0.8515583f32],
        tex_coords: [-1.3096542f32, 1.3515583f32],
    },
    Vertex {
        position  : [-1.8595531f32, 0.7362488f32],
        tex_coords: [-1.3595531f32, 1.2362487f32],
    },
    Vertex {
        position  : [-1.9021132f32, 0.6180336f32],
        tex_coords: [-1.4021132f32, 1.1180336f32],
    },
    Vertex {
        position  : [-1.9371663f32, 0.4973798f32],
        tex_coords: [-1.4371663f32, 0.9973798f32],
    },
    Vertex {
        position  : [-1.9645746f32, 0.37476215f32],
        tex_coords: [-1.4645746f32, 0.8747622f32],
    },
    Vertex {
        position  : [-1.9842294f32, 0.2506664f32],
        tex_coords: [-1.4842294f32, 0.7506664f32],
    },
    Vertex {
        position  : [-1.9960535f32, 0.12558044f32],
        tex_coords: [-1.4960535f32, 0.62558043f32],
    },
    Vertex {
        position  : [-2f32, -0.00000017484555f32],
        tex_coords: [-1.5f32, 0.49999982f32],
    },
    Vertex {
        position  : [-1.9960535f32, -0.12558174f32],
        tex_coords: [-1.4960535f32, 0.37441826f32],
    },
    Vertex {
        position  : [-1.9842293f32, -0.25066674f32],
        tex_coords: [-1.4842293f32, 0.24933326f32],
    },
    Vertex {
        position  : [-1.9645746f32, -0.37476248f32],
        tex_coords: [-1.4645746f32, 0.12523752f32],
    },
    Vertex {
        position  : [-1.9371662f32, -0.49738014f32],
        tex_coords: [-1.4371662f32, 0.0026198626f32],
    },
    Vertex {
        position  : [-1.9021131f32, -0.61803395f32],
        tex_coords: [-1.4021131f32, -0.118033946f32],
    },
    Vertex {
        position  : [-1.8595527f32, -0.73624957f32],
        tex_coords: [-1.3595527f32, -0.23624957f32],
    },
    Vertex {
        position  : [-1.8096541f32, -0.8515586f32],
        tex_coords: [-1.3096541f32, -0.35155863f32],
    },
    Vertex {
        position  : [-1.7526133f32, -0.9635074f32],
        tex_coords: [-1.2526133f32, -0.4635074f32],
    },
    Vertex {
        position  : [-1.6886557f32, -1.0716537f32],
        tex_coords: [-1.1886557f32, -0.5716537f32],
    },
    Vertex {
        position  : [-1.6180339f32, -1.1755707f32],
        tex_coords: [-1.1180339f32, -0.6755707f32],
    },
    Vertex {
        position  : [-1.5410264f32, -1.2748482f32],
        tex_coords: [-1.0410264f32, -0.7748482f32],
    },
    Vertex {
        position  : [-1.457937f32, -1.3690945f32],
        tex_coords: [-0.957937f32, -0.8690945f32],
    },
    Vertex {
        position  : [-1.3690939f32, -1.4579375f32],
        tex_coords: [-0.8690939f32, -0.9579375f32],
    },
    Vertex {
        position  : [-1.2748476f32, -1.5410267f32],
        tex_coords: [-0.7748476f32, -1.0410267f32],
    },
    Vertex {
        position  : [-1.1755708f32, -1.6180336f32],
        tex_coords: [-0.67557085f32, -1.1180336f32],
    },
    Vertex {
        position  : [-1.0716531f32, -1.6886561f32],
        tex_coords: [-0.5716531f32, -1.1886561f32],
    },
    Vertex {
        position  : [-0.9635077f32, -1.7526132f32],
        tex_coords: [-0.4635077f32, -1.2526132f32],
    },
    Vertex {
        position  : [-0.851558f32, -1.8096544f32],
        tex_coords: [-0.35155803f32, -1.3096544f32],
    },
    Vertex {
        position  : [-0.7362494f32, -1.8595529f32],
        tex_coords: [-0.23624939f32, -1.3595529f32],
    },
    Vertex {
        position  : [-0.6180333f32, -1.9021132f32],
        tex_coords: [-0.11803329f32, -1.4021132f32],
    },
    Vertex {
        position  : [-0.49737996f32, -1.9371663f32],
        tex_coords: [0.0026200414f32, -1.4371663f32],
    },
    Vertex {
        position  : [-0.37476274f32, -1.9645745f32],
        tex_coords: [0.12523726f32, -1.4645745f32],
    },
    Vertex {
        position  : [-0.25066656f32, -1.9842294f32],
        tex_coords: [0.24933344f32, -1.4842294f32],
    },
    Vertex {
        position  : [-0.12558107f32, -1.9960535f32],
        tex_coords: [0.3744189f32, -1.4960535f32],
    },
    Vertex {
        position  : [0.000000023849761f32, -2f32],
        tex_coords: [0.5f32, -1.5f32],
    },
    Vertex {
        position  : [0.12558112f32, -1.9960535f32],
        tex_coords: [0.62558115f32, -1.4960535f32],
    },
    Vertex {
        position  : [0.2506666f32, -1.9842294f32],
        tex_coords: [0.7506666f32, -1.4842294f32],
    },
    Vertex {
        position  : [0.3747628f32, -1.9645745f32],
        tex_coords: [0.8747628f32, -1.4645745f32],
    },
    Vertex {
        position  : [0.49738f32, -1.9371662f32],
        tex_coords: [0.99738f32, -1.4371662f32],
    },
    Vertex {
        position  : [0.61803424f32, -1.902113f32],
        tex_coords: [1.1180342f32, -1.402113f32],
    },
    Vertex {
        position  : [0.7362494f32, -1.8595529f32],
        tex_coords: [1.2362494f32, -1.3595529f32],
    },
    Vertex {
        position  : [0.8515589f32, -1.809654f32],
        tex_coords: [1.3515589f32, -1.309654f32],
    },
    Vertex {
        position  : [0.9635077f32, -1.7526132f32],
        tex_coords: [1.4635077f32, -1.2526132f32],
    },
    Vertex {
        position  : [1.071654f32, -1.6886556f32],
        tex_coords: [1.571654f32, -1.1886556f32],
    },
    Vertex {
        position  : [1.1755701f32, -1.6180342f32],
        tex_coords: [1.6755701f32, -1.1180342f32],
    },
    Vertex {
        position  : [1.2748485f32, -1.5410261f32],
        tex_coords: [1.7748485f32, -1.0410261f32],
    },
    Vertex {
        position  : [1.3690947f32, -1.4579368f32],
        tex_coords: [1.8690947f32, -0.95793676f32],
    },
    Vertex {
        position  : [1.4579377f32, -1.3690937f32],
        tex_coords: [1.9579377f32, -0.86909366f32],
    },
    Vertex {
        position  : [1.5410264f32, -1.2748481f32],
        tex_coords: [2.0410264f32, -0.7748481f32],
    },
    Vertex {
        position  : [1.6180345f32, -1.1755699f32],
        tex_coords: [2.1180344f32, -0.6755699f32],
    },
    Vertex {
        position  : [1.6886563f32, -1.0716529f32],
        tex_coords: [2.1886563f32, -0.5716529f32],
    },
    Vertex {
        position  : [1.7526133f32, -0.9635074f32],
        tex_coords: [2.2526133f32, -0.4635074f32],
    },
    Vertex {
        position  : [1.8096541f32, -0.85155857f32],
        tex_coords: [2.3096542f32, -0.35155857f32],
    },
    Vertex {
        position  : [1.8595533f32, -0.7362482f32],
        tex_coords: [2.3595533f32, -0.2362482f32],
    },
    Vertex {
        position  : [1.9021133f32, -0.618033f32],
        tex_coords: [2.4021134f32, -0.11803299f32],
    },
    Vertex {
        position  : [1.9371663f32, -0.49737963f32],
        tex_coords: [2.4371662f32, 0.0026203692f32],
    },
    Vertex {
        position  : [1.9645746f32, -0.37476245f32],
        tex_coords: [2.4645746f32, 0.12523755f32],
    },
    Vertex {
        position  : [1.9842296f32, -0.25066528f32],
        tex_coords: [2.4842296f32, 0.24933472f32],
    },
    Vertex {
        position  : [1.9960535f32, -0.12558074f32],
        tex_coords: [2.4960535f32, 0.37441927f32],
    },
];