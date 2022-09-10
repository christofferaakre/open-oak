use crate::structs::Vertex;
/// Vertices for rendering a circle.
pub const VERTICES: [Vertex; 100] = [
    Vertex {
        position  : [0.5f32, 0f32],
        tex_coords: [1f32, 0.5f32],
    },
    Vertex {
        position  : [0.49901336f32, 0.03139526f32],
        tex_coords: [0.99901336f32, 0.53139526f32],
    },
    Vertex {
        position  : [0.49605736f32, 0.06266662f32],
        tex_coords: [0.9960574f32, 0.5626666f32],
    },
    Vertex {
        position  : [0.4911436f32, 0.093690656f32],
        tex_coords: [0.9911436f32, 0.59369063f32],
    },
    Vertex {
        position  : [0.48429158f32, 0.12434495f32],
        tex_coords: [0.98429155f32, 0.62434494f32],
    },
    Vertex {
        position  : [0.47552827f32, 0.1545085f32],
        tex_coords: [0.97552824f32, 0.6545085f32],
    },
    Vertex {
        position  : [0.46488824f32, 0.18406227f32],
        tex_coords: [0.9648882f32, 0.68406224f32],
    },
    Vertex {
        position  : [0.45241353f32, 0.21288966f32],
        tex_coords: [0.95241356f32, 0.7128897f32],
    },
    Vertex {
        position  : [0.43815333f32, 0.24087685f32],
        tex_coords: [0.9381533f32, 0.74087685f32],
    },
    Vertex {
        position  : [0.42216396f32, 0.2679134f32],
        tex_coords: [0.92216396f32, 0.7679134f32],
    },
    Vertex {
        position  : [0.4045085f32, 0.29389262f32],
        tex_coords: [0.9045085f32, 0.7938926f32],
    },
    Vertex {
        position  : [0.38525662f32, 0.31871203f32],
        tex_coords: [0.88525665f32, 0.818712f32],
    },
    Vertex {
        position  : [0.3644843f32, 0.34227356f32],
        tex_coords: [0.8644843f32, 0.8422736f32],
    },
    Vertex {
        position  : [0.34227353f32, 0.36448434f32],
        tex_coords: [0.84227353f32, 0.8644843f32],
    },
    Vertex {
        position  : [0.31871197f32, 0.38525665f32],
        tex_coords: [0.818712f32, 0.88525665f32],
    },
    Vertex {
        position  : [0.29389262f32, 0.4045085f32],
        tex_coords: [0.7938926f32, 0.9045085f32],
    },
    Vertex {
        position  : [0.26791337f32, 0.42216396f32],
        tex_coords: [0.76791334f32, 0.92216396f32],
    },
    Vertex {
        position  : [0.24087681f32, 0.43815336f32],
        tex_coords: [0.7408768f32, 0.9381534f32],
    },
    Vertex {
        position  : [0.21288966f32, 0.45241353f32],
        tex_coords: [0.7128897f32, 0.95241356f32],
    },
    Vertex {
        position  : [0.18406227f32, 0.46488824f32],
        tex_coords: [0.68406224f32, 0.9648882f32],
    },
    Vertex {
        position  : [0.15450849f32, 0.47552827f32],
        tex_coords: [0.6545085f32, 0.97552824f32],
    },
    Vertex {
        position  : [0.124344915f32, 0.48429158f32],
        tex_coords: [0.62434494f32, 0.98429155f32],
    },
    Vertex {
        position  : [0.09369062f32, 0.49114364f32],
        tex_coords: [0.59369063f32, 0.99114364f32],
    },
    Vertex {
        position  : [0.06266662f32, 0.49605736f32],
        tex_coords: [0.5626666f32, 0.9960574f32],
    },
    Vertex {
        position  : [0.03139525f32, 0.49901336f32],
        tex_coords: [0.53139526f32, 0.99901336f32],
    },
    Vertex {
        position  : [-0.000000021855694f32, 0.5f32],
        tex_coords: [0.49999997f32, 1f32],
    },
    Vertex {
        position  : [-0.031395294f32, 0.49901336f32],
        tex_coords: [0.4686047f32, 0.99901336f32],
    },
    Vertex {
        position  : [-0.06266666f32, 0.49605733f32],
        tex_coords: [0.43733335f32, 0.99605733f32],
    },
    Vertex {
        position  : [-0.093690716f32, 0.4911436f32],
        tex_coords: [0.40630928f32, 0.9911436f32],
    },
    Vertex {
        position  : [-0.12434496f32, 0.48429158f32],
        tex_coords: [0.37565506f32, 0.98429155f32],
    },
    Vertex {
        position  : [-0.15450852f32, 0.47552824f32],
        tex_coords: [0.34549147f32, 0.97552824f32],
    },
    Vertex {
        position  : [-0.18406232f32, 0.46488822f32],
        tex_coords: [0.3159377f32, 0.9648882f32],
    },
    Vertex {
        position  : [-0.21288969f32, 0.4524135f32],
        tex_coords: [0.28711033f32, 0.9524135f32],
    },
    Vertex {
        position  : [-0.2408769f32, 0.4381533f32],
        tex_coords: [0.2591231f32, 0.93815327f32],
    },
    Vertex {
        position  : [-0.26791346f32, 0.42216393f32],
        tex_coords: [0.23208654f32, 0.92216396f32],
    },
    Vertex {
        position  : [-0.2938927f32, 0.40450844f32],
        tex_coords: [0.20610729f32, 0.9045085f32],
    },
    Vertex {
        position  : [-0.318712f32, 0.38525662f32],
        tex_coords: [0.181288f32, 0.88525665f32],
    },
    Vertex {
        position  : [-0.34227356f32, 0.3644843f32],
        tex_coords: [0.15772644f32, 0.8644843f32],
    },
    Vertex {
        position  : [-0.3644843f32, 0.34227353f32],
        tex_coords: [0.13551569f32, 0.84227353f32],
    },
    Vertex {
        position  : [-0.38525665f32, 0.31871197f32],
        tex_coords: [0.11474335f32, 0.818712f32],
    },
    Vertex {
        position  : [-0.40450853f32, 0.2938926f32],
        tex_coords: [0.09549147f32, 0.7938926f32],
    },
    Vertex {
        position  : [-0.422164f32, 0.26791334f32],
        tex_coords: [0.07783601f32, 0.76791334f32],
    },
    Vertex {
        position  : [-0.43815336f32, 0.24087678f32],
        tex_coords: [0.061846644f32, 0.7408768f32],
    },
    Vertex {
        position  : [-0.45241356f32, 0.21288958f32],
        tex_coords: [0.04758644f32, 0.71288955f32],
    },
    Vertex {
        position  : [-0.46488827f32, 0.1840622f32],
        tex_coords: [0.035111725f32, 0.6840622f32],
    },
    Vertex {
        position  : [-0.4755283f32, 0.1545084f32],
        tex_coords: [0.0244717f32, 0.6545084f32],
    },
    Vertex {
        position  : [-0.48429158f32, 0.12434495f32],
        tex_coords: [0.015708417f32, 0.62434494f32],
    },
    Vertex {
        position  : [-0.49114364f32, 0.09369054f32],
        tex_coords: [0.008856356f32, 0.5936905f32],
    },
    Vertex {
        position  : [-0.49605736f32, 0.0626666f32],
        tex_coords: [0.0039426386f32, 0.5626666f32],
    },
    Vertex {
        position  : [-0.49901336f32, 0.03139511f32],
        tex_coords: [0.0009866357f32, 0.53139514f32],
    },
    Vertex {
        position  : [-0.5f32, -0.00000004371139f32],
        tex_coords: [0f32, 0.49999997f32],
    },
    Vertex {
        position  : [-0.49901336f32, -0.031395435f32],
        tex_coords: [0.0009866357f32, 0.46860456f32],
    },
    Vertex {
        position  : [-0.49605733f32, -0.062666684f32],
        tex_coords: [0.0039426684f32, 0.43733332f32],
    },
    Vertex {
        position  : [-0.49114364f32, -0.09369062f32],
        tex_coords: [0.008856356f32, 0.40630937f32],
    },
    Vertex {
        position  : [-0.48429155f32, -0.124345034f32],
        tex_coords: [0.015708447f32, 0.37565497f32],
    },
    Vertex {
        position  : [-0.47552827f32, -0.15450849f32],
        tex_coords: [0.02447173f32, 0.34549153f32],
    },
    Vertex {
        position  : [-0.4648882f32, -0.18406239f32],
        tex_coords: [0.035111815f32, 0.3159376f32],
    },
    Vertex {
        position  : [-0.45241353f32, -0.21288966f32],
        tex_coords: [0.04758647f32, 0.28711033f32],
    },
    Vertex {
        position  : [-0.43815333f32, -0.24087685f32],
        tex_coords: [0.061846673f32, 0.25912315f32],
    },
    Vertex {
        position  : [-0.42216393f32, -0.26791343f32],
        tex_coords: [0.07783607f32, 0.23208657f32],
    },
    Vertex {
        position  : [-0.40450847f32, -0.29389268f32],
        tex_coords: [0.09549153f32, 0.20610732f32],
    },
    Vertex {
        position  : [-0.3852566f32, -0.31871206f32],
        tex_coords: [0.11474341f32, 0.18128794f32],
    },
    Vertex {
        position  : [-0.36448425f32, -0.34227362f32],
        tex_coords: [0.13551575f32, 0.15772638f32],
    },
    Vertex {
        position  : [-0.34227347f32, -0.36448437f32],
        tex_coords: [0.15772653f32, 0.13551563f32],
    },
    Vertex {
        position  : [-0.3187119f32, -0.38525668f32],
        tex_coords: [0.1812881f32, 0.11474332f32],
    },
    Vertex {
        position  : [-0.2938927f32, -0.4045084f32],
        tex_coords: [0.20610729f32, 0.09549159f32],
    },
    Vertex {
        position  : [-0.26791328f32, -0.42216402f32],
        tex_coords: [0.23208672f32, 0.07783598f32],
    },
    Vertex {
        position  : [-0.24087693f32, -0.4381533f32],
        tex_coords: [0.2591231f32, 0.061846703f32],
    },
    Vertex {
        position  : [-0.2128895f32, -0.4524136f32],
        tex_coords: [0.2871105f32, 0.04758641f32],
    },
    Vertex {
        position  : [-0.18406235f32, -0.46488822f32],
        tex_coords: [0.31593764f32, 0.035111785f32],
    },
    Vertex {
        position  : [-0.15450832f32, -0.4755283f32],
        tex_coords: [0.34549168f32, 0.0244717f32],
    },
    Vertex {
        position  : [-0.12434499f32, -0.48429158f32],
        tex_coords: [0.375655f32, 0.015708417f32],
    },
    Vertex {
        position  : [-0.093690686f32, -0.4911436f32],
        tex_coords: [0.4063093f32, 0.008856386f32],
    },
    Vertex {
        position  : [-0.06266664f32, -0.49605736f32],
        tex_coords: [0.43733335f32, 0.0039426386f32],
    },
    Vertex {
        position  : [-0.031395268f32, -0.49901336f32],
        tex_coords: [0.46860474f32, 0.0009866357f32],
    },
    Vertex {
        position  : [0.0000000059624403f32, -0.5f32],
        tex_coords: [0.5f32, 0f32],
    },
    Vertex {
        position  : [0.03139528f32, -0.49901336f32],
        tex_coords: [0.53139526f32, 0.0009866357f32],
    },
    Vertex {
        position  : [0.06266665f32, -0.49605736f32],
        tex_coords: [0.56266665f32, 0.0039426386f32],
    },
    Vertex {
        position  : [0.0936907f32, -0.4911436f32],
        tex_coords: [0.5936907f32, 0.008856386f32],
    },
    Vertex {
        position  : [0.124345f32, -0.48429155f32],
        tex_coords: [0.624345f32, 0.015708447f32],
    },
    Vertex {
        position  : [0.15450856f32, -0.47552824f32],
        tex_coords: [0.6545086f32, 0.02447176f32],
    },
    Vertex {
        position  : [0.18406235f32, -0.46488822f32],
        tex_coords: [0.68406236f32, 0.035111785f32],
    },
    Vertex {
        position  : [0.21288973f32, -0.4524135f32],
        tex_coords: [0.71288973f32, 0.0475865f32],
    },
    Vertex {
        position  : [0.24087693f32, -0.4381533f32],
        tex_coords: [0.7408769f32, 0.061846703f32],
    },
    Vertex {
        position  : [0.2679135f32, -0.4221639f32],
        tex_coords: [0.76791346f32, 0.0778361f32],
    },
    Vertex {
        position  : [0.29389253f32, -0.40450856f32],
        tex_coords: [0.7938925f32, 0.09549144f32],
    },
    Vertex {
        position  : [0.31871212f32, -0.38525653f32],
        tex_coords: [0.8187121f32, 0.11474347f32],
    },
    Vertex {
        position  : [0.34227368f32, -0.3644842f32],
        tex_coords: [0.8422737f32, 0.13551581f32],
    },
    Vertex {
        position  : [0.36448443f32, -0.3422734f32],
        tex_coords: [0.8644844f32, 0.15772659f32],
    },
    Vertex {
        position  : [0.3852566f32, -0.31871203f32],
        tex_coords: [0.8852566f32, 0.18128797f32],
    },
    Vertex {
        position  : [0.40450862f32, -0.29389247f32],
        tex_coords: [0.9045086f32, 0.20610753f32],
    },
    Vertex {
        position  : [0.42216408f32, -0.26791322f32],
        tex_coords: [0.9221641f32, 0.23208678f32],
    },
    Vertex {
        position  : [0.43815333f32, -0.24087685f32],
        tex_coords: [0.9381533f32, 0.25912315f32],
    },
    Vertex {
        position  : [0.45241353f32, -0.21288964f32],
        tex_coords: [0.95241356f32, 0.28711036f32],
    },
    Vertex {
        position  : [0.46488833f32, -0.18406205f32],
        tex_coords: [0.96488833f32, 0.31593794f32],
    },
    Vertex {
        position  : [0.47552833f32, -0.15450825f32],
        tex_coords: [0.97552836f32, 0.34549177f32],
    },
    Vertex {
        position  : [0.48429158f32, -0.12434491f32],
        tex_coords: [0.98429155f32, 0.37565508f32],
    },
    Vertex {
        position  : [0.49114364f32, -0.09369061f32],
        tex_coords: [0.99114364f32, 0.4063094f32],
    },
    Vertex {
        position  : [0.4960574f32, -0.06266632f32],
        tex_coords: [0.9960574f32, 0.43733367f32],
    },
    Vertex {
        position  : [0.49901336f32, -0.031395186f32],
        tex_coords: [0.99901336f32, 0.4686048f32],
    },
];