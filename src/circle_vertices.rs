use crate::structs::Vertex;
/// Vertices for rendering a circle.
pub const VERTICES: [Vertex; 100] = [
    Vertex {
        position  : [1f32, 0f32],
        tex_coords: [1.5f32, 0.5f32],
    },
    Vertex {
        position  : [0.9980267f32, 0.06279052f32],
        tex_coords: [1.4980267f32, 0.5627905f32],
    },
    Vertex {
        position  : [0.9921147f32, 0.12533323f32],
        tex_coords: [1.4921148f32, 0.62533325f32],
    },
    Vertex {
        position  : [0.9822872f32, 0.18738131f32],
        tex_coords: [1.4822872f32, 0.6873813f32],
    },
    Vertex {
        position  : [0.96858317f32, 0.2486899f32],
        tex_coords: [1.4685831f32, 0.7486899f32],
    },
    Vertex {
        position  : [0.95105654f32, 0.309017f32],
        tex_coords: [1.4510565f32, 0.809017f32],
    },
    Vertex {
        position  : [0.9297765f32, 0.36812454f32],
        tex_coords: [1.4297764f32, 0.86812454f32],
    },
    Vertex {
        position  : [0.90482706f32, 0.4257793f32],
        tex_coords: [1.4048271f32, 0.92577934f32],
    },
    Vertex {
        position  : [0.87630665f32, 0.4817537f32],
        tex_coords: [1.3763067f32, 0.9817537f32],
    },
    Vertex {
        position  : [0.8443279f32, 0.5358268f32],
        tex_coords: [1.3443279f32, 1.0358268f32],
    },
    Vertex {
        position  : [0.809017f32, 0.58778524f32],
        tex_coords: [1.309017f32, 1.0877852f32],
    },
    Vertex {
        position  : [0.77051324f32, 0.63742405f32],
        tex_coords: [1.2705133f32, 1.137424f32],
    },
    Vertex {
        position  : [0.7289686f32, 0.6845471f32],
        tex_coords: [1.2289686f32, 1.1845472f32],
    },
    Vertex {
        position  : [0.68454707f32, 0.7289687f32],
        tex_coords: [1.1845471f32, 1.2289686f32],
    },
    Vertex {
        position  : [0.63742393f32, 0.7705133f32],
        tex_coords: [1.137424f32, 1.2705133f32],
    },
    Vertex {
        position  : [0.58778524f32, 0.809017f32],
        tex_coords: [1.0877852f32, 1.309017f32],
    },
    Vertex {
        position  : [0.53582674f32, 0.8443279f32],
        tex_coords: [1.0358267f32, 1.3443279f32],
    },
    Vertex {
        position  : [0.48175362f32, 0.8763067f32],
        tex_coords: [0.9817536f32, 1.3763068f32],
    },
    Vertex {
        position  : [0.4257793f32, 0.90482706f32],
        tex_coords: [0.92577934f32, 1.4048271f32],
    },
    Vertex {
        position  : [0.36812454f32, 0.9297765f32],
        tex_coords: [0.86812454f32, 1.4297764f32],
    },
    Vertex {
        position  : [0.30901697f32, 0.95105654f32],
        tex_coords: [0.80901694f32, 1.4510565f32],
    },
    Vertex {
        position  : [0.24868983f32, 0.96858317f32],
        tex_coords: [0.74868983f32, 1.4685831f32],
    },
    Vertex {
        position  : [0.18738124f32, 0.9822873f32],
        tex_coords: [0.68738127f32, 1.4822873f32],
    },
    Vertex {
        position  : [0.12533323f32, 0.9921147f32],
        tex_coords: [0.62533325f32, 1.4921148f32],
    },
    Vertex {
        position  : [0.0627905f32, 0.9980267f32],
        tex_coords: [0.5627905f32, 1.4980267f32],
    },
    Vertex {
        position  : [-0.00000004371139f32, 1f32],
        tex_coords: [0.49999997f32, 1.5f32],
    },
    Vertex {
        position  : [-0.06279059f32, 0.9980267f32],
        tex_coords: [0.43720943f32, 1.4980267f32],
    },
    Vertex {
        position  : [-0.12533332f32, 0.99211466f32],
        tex_coords: [0.3746667f32, 1.4921147f32],
    },
    Vertex {
        position  : [-0.18738143f32, 0.9822872f32],
        tex_coords: [0.31261855f32, 1.4822872f32],
    },
    Vertex {
        position  : [-0.24868992f32, 0.96858317f32],
        tex_coords: [0.25131008f32, 1.4685831f32],
    },
    Vertex {
        position  : [-0.30901703f32, 0.9510565f32],
        tex_coords: [0.19098297f32, 1.4510565f32],
    },
    Vertex {
        position  : [-0.36812463f32, 0.92977643f32],
        tex_coords: [0.13187537f32, 1.4297764f32],
    },
    Vertex {
        position  : [-0.42577937f32, 0.904827f32],
        tex_coords: [0.07422063f32, 1.404827f32],
    },
    Vertex {
        position  : [-0.4817538f32, 0.8763066f32],
        tex_coords: [0.018246204f32, 1.3763065f32],
    },
    Vertex {
        position  : [-0.5358269f32, 0.84432787f32],
        tex_coords: [-0.03582692f32, 1.3443279f32],
    },
    Vertex {
        position  : [-0.5877854f32, 0.8090169f32],
        tex_coords: [-0.08778542f32, 1.309017f32],
    },
    Vertex {
        position  : [-0.637424f32, 0.77051324f32],
        tex_coords: [-0.13742399f32, 1.2705133f32],
    },
    Vertex {
        position  : [-0.6845471f32, 0.7289686f32],
        tex_coords: [-0.18454713f32, 1.2289686f32],
    },
    Vertex {
        position  : [-0.7289686f32, 0.68454707f32],
        tex_coords: [-0.22896862f32, 1.1845471f32],
    },
    Vertex {
        position  : [-0.7705133f32, 0.63742393f32],
        tex_coords: [-0.2705133f32, 1.137424f32],
    },
    Vertex {
        position  : [-0.80901706f32, 0.5877852f32],
        tex_coords: [-0.30901706f32, 1.0877852f32],
    },
    Vertex {
        position  : [-0.844328f32, 0.5358267f32],
        tex_coords: [-0.344328f32, 1.0358267f32],
    },
    Vertex {
        position  : [-0.8763067f32, 0.48175356f32],
        tex_coords: [-0.3763067f32, 0.9817536f32],
    },
    Vertex {
        position  : [-0.9048271f32, 0.42577916f32],
        tex_coords: [-0.40482712f32, 0.92577916f32],
    },
    Vertex {
        position  : [-0.92977655f32, 0.3681244f32],
        tex_coords: [-0.42977655f32, 0.86812437f32],
    },
    Vertex {
        position  : [-0.9510566f32, 0.3090168f32],
        tex_coords: [-0.4510566f32, 0.8090168f32],
    },
    Vertex {
        position  : [-0.96858317f32, 0.2486899f32],
        tex_coords: [-0.46858317f32, 0.7486899f32],
    },
    Vertex {
        position  : [-0.9822873f32, 0.18738107f32],
        tex_coords: [-0.4822873f32, 0.6873811f32],
    },
    Vertex {
        position  : [-0.9921147f32, 0.1253332f32],
        tex_coords: [-0.49211472f32, 0.6253332f32],
    },
    Vertex {
        position  : [-0.9980267f32, 0.06279022f32],
        tex_coords: [-0.49802673f32, 0.5627902f32],
    },
    Vertex {
        position  : [-1f32, -0.00000008742278f32],
        tex_coords: [-0.5f32, 0.4999999f32],
    },
    Vertex {
        position  : [-0.9980267f32, -0.06279087f32],
        tex_coords: [-0.49802673f32, 0.43720913f32],
    },
    Vertex {
        position  : [-0.99211466f32, -0.12533337f32],
        tex_coords: [-0.49211466f32, 0.37466663f32],
    },
    Vertex {
        position  : [-0.9822873f32, -0.18738124f32],
        tex_coords: [-0.4822873f32, 0.31261876f32],
    },
    Vertex {
        position  : [-0.9685831f32, -0.24869007f32],
        tex_coords: [-0.4685831f32, 0.25130993f32],
    },
    Vertex {
        position  : [-0.95105654f32, -0.30901697f32],
        tex_coords: [-0.45105654f32, 0.19098303f32],
    },
    Vertex {
        position  : [-0.9297764f32, -0.36812478f32],
        tex_coords: [-0.42977637f32, 0.13187522f32],
    },
    Vertex {
        position  : [-0.90482706f32, -0.4257793f32],
        tex_coords: [-0.40482706f32, 0.07422069f32],
    },
    Vertex {
        position  : [-0.87630665f32, -0.4817537f32],
        tex_coords: [-0.37630665f32, 0.018246293f32],
    },
    Vertex {
        position  : [-0.84432787f32, -0.53582686f32],
        tex_coords: [-0.34432787f32, -0.03582686f32],
    },
    Vertex {
        position  : [-0.80901694f32, -0.58778536f32],
        tex_coords: [-0.30901694f32, -0.08778536f32],
    },
    Vertex {
        position  : [-0.7705132f32, -0.6374241f32],
        tex_coords: [-0.27051318f32, -0.13742411f32],
    },
    Vertex {
        position  : [-0.7289685f32, -0.68454725f32],
        tex_coords: [-0.2289685f32, -0.18454725f32],
    },
    Vertex {
        position  : [-0.68454695f32, -0.72896874f32],
        tex_coords: [-0.18454695f32, -0.22896874f32],
    },
    Vertex {
        position  : [-0.6374238f32, -0.77051336f32],
        tex_coords: [-0.13742381f32, -0.27051336f32],
    },
    Vertex {
        position  : [-0.5877854f32, -0.8090168f32],
        tex_coords: [-0.08778542f32, -0.30901682f32],
    },
    Vertex {
        position  : [-0.53582656f32, -0.84432805f32],
        tex_coords: [-0.035826564f32, -0.34432805f32],
    },
    Vertex {
        position  : [-0.48175386f32, -0.8763066f32],
        tex_coords: [0.018246144f32, -0.3763066f32],
    },
    Vertex {
        position  : [-0.425779f32, -0.9048272f32],
        tex_coords: [0.074220985f32, -0.40482718f32],
    },
    Vertex {
        position  : [-0.3681247f32, -0.92977643f32],
        tex_coords: [0.1318753f32, -0.42977643f32],
    },
    Vertex {
        position  : [-0.30901664f32, -0.9510566f32],
        tex_coords: [0.19098336f32, -0.4510566f32],
    },
    Vertex {
        position  : [-0.24868998f32, -0.96858317f32],
        tex_coords: [0.25131002f32, -0.46858317f32],
    },
    Vertex {
        position  : [-0.18738137f32, -0.9822872f32],
        tex_coords: [0.3126186f32, -0.48228723f32],
    },
    Vertex {
        position  : [-0.12533328f32, -0.9921147f32],
        tex_coords: [0.37466672f32, -0.49211472f32],
    },
    Vertex {
        position  : [-0.062790535f32, -0.9980267f32],
        tex_coords: [0.43720946f32, -0.49802673f32],
    },
    Vertex {
        position  : [0.000000011924881f32, -1f32],
        tex_coords: [0.5f32, -0.5f32],
    },
    Vertex {
        position  : [0.06279056f32, -0.9980267f32],
        tex_coords: [0.5627906f32, -0.49802673f32],
    },
    Vertex {
        position  : [0.1253333f32, -0.9921147f32],
        tex_coords: [0.6253333f32, -0.49211472f32],
    },
    Vertex {
        position  : [0.1873814f32, -0.9822872f32],
        tex_coords: [0.6873814f32, -0.48228723f32],
    },
    Vertex {
        position  : [0.24869f32, -0.9685831f32],
        tex_coords: [0.74869f32, -0.4685831f32],
    },
    Vertex {
        position  : [0.30901712f32, -0.9510565f32],
        tex_coords: [0.8090171f32, -0.45105648f32],
    },
    Vertex {
        position  : [0.3681247f32, -0.92977643f32],
        tex_coords: [0.8681247f32, -0.42977643f32],
    },
    Vertex {
        position  : [0.42577946f32, -0.904827f32],
        tex_coords: [0.92577946f32, -0.404827f32],
    },
    Vertex {
        position  : [0.48175386f32, -0.8763066f32],
        tex_coords: [0.9817538f32, -0.3763066f32],
    },
    Vertex {
        position  : [0.535827f32, -0.8443278f32],
        tex_coords: [1.0358269f32, -0.3443278f32],
    },
    Vertex {
        position  : [0.58778507f32, -0.8090171f32],
        tex_coords: [1.087785f32, -0.30901712f32],
    },
    Vertex {
        position  : [0.63742423f32, -0.77051306f32],
        tex_coords: [1.1374242f32, -0.27051306f32],
    },
    Vertex {
        position  : [0.68454736f32, -0.7289684f32],
        tex_coords: [1.1845474f32, -0.22896838f32],
    },
    Vertex {
        position  : [0.72896886f32, -0.6845468f32],
        tex_coords: [1.2289689f32, -0.18454683f32],
    },
    Vertex {
        position  : [0.7705132f32, -0.63742405f32],
        tex_coords: [1.2705132f32, -0.13742405f32],
    },
    Vertex {
        position  : [0.80901724f32, -0.58778495f32],
        tex_coords: [1.3090172f32, -0.087784946f32],
    },
    Vertex {
        position  : [0.84432817f32, -0.53582644f32],
        tex_coords: [1.3443282f32, -0.035826445f32],
    },
    Vertex {
        position  : [0.87630665f32, -0.4817537f32],
        tex_coords: [1.3763067f32, 0.018246293f32],
    },
    Vertex {
        position  : [0.90482706f32, -0.42577928f32],
        tex_coords: [1.4048271f32, 0.07422072f32],
    },
    Vertex {
        position  : [0.92977667f32, -0.3681241f32],
        tex_coords: [1.4297767f32, 0.1318759f32],
    },
    Vertex {
        position  : [0.95105666f32, -0.3090165f32],
        tex_coords: [1.4510567f32, 0.1909835f32],
    },
    Vertex {
        position  : [0.96858317f32, -0.24868982f32],
        tex_coords: [1.4685831f32, 0.25131017f32],
    },
    Vertex {
        position  : [0.9822873f32, -0.18738122f32],
        tex_coords: [1.4822873f32, 0.3126188f32],
    },
    Vertex {
        position  : [0.9921148f32, -0.12533264f32],
        tex_coords: [1.4921148f32, 0.37466735f32],
    },
    Vertex {
        position  : [0.9980267f32, -0.06279037f32],
        tex_coords: [1.4980267f32, 0.43720964f32],
    },
];