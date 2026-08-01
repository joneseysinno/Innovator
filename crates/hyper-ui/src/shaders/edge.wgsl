struct Globals {
    screen_size: vec2<f32>,
    time: f32,
    _pad: f32,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @location(0) p0: vec2<f32>,
    @location(1) p1: vec2<f32>,
    @location(2) p2: vec2<f32>,
    @location(3) p3: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) width: f32,
    @location(6) edge_kind: u32,
    @location(7) arrow: u32,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) along: f32,
    @location(2) edge_kind: f32,
};

fn bezier(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let u = 1.0 - t;
    return u * u * u * p0
        + 3.0 * u * u * t * p1
        + 3.0 * u * t * t * p2
        + t * t * t * p3;
}

fn bezier_deriv(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let u = 1.0 - t;
    return 3.0 * u * u * (p1 - p0)
        + 6.0 * u * t * (p2 - p1)
        + 3.0 * t * t * (p3 - p2);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let segments = 32u;
    let vi = in.vertex_index;
    let seg = vi / 2u;
    let side = select(-1.0, 1.0, (vi % 2u) == 1u);
    let t = f32(min(seg, segments)) / f32(segments);

    let pos = bezier(in.p0, in.p1, in.p2, in.p3, t);
    var tang = bezier_deriv(in.p0, in.p1, in.p2, in.p3, t);
    let len = max(length(tang), 0.0001);
    tang = tang / len;
    let normal = vec2<f32>(-tang.y, tang.x);

    // Arrowhead: expand last few segments into a triangle tip
    var half_w = in.width * 0.5;
    if in.arrow != 0u && t > 0.88 {
        half_w = mix(half_w, in.width * 2.5, (t - 0.88) / 0.12);
    }

    let screen = pos + normal * half_w * side;
    let ndc = vec2<f32>(
        (screen.x / globals.screen_size.x) * 2.0 - 1.0,
        1.0 - (screen.y / globals.screen_size.y) * 2.0,
    );
    out.clip_position = vec4<f32>(ndc, 0.0, 1.0);
    out.color = in.color;
    out.along = t;
    out.edge_kind = f32(in.edge_kind);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var alpha = in.color.a;
    let kind = u32(in.edge_kind + 0.5);

    // Signal = solid
    // Stream = animated dash
    // Wave = pulsing opacity
    // Binding = solid slightly muted
    if kind == 1u {
        let dash = fract(in.along * 16.0 - globals.time * 2.0);
        if dash > 0.55 {
            discard;
        }
    } else if kind == 2u {
        alpha *= 0.55 + 0.45 * sin(globals.time * 4.0 + in.along * 12.0);
    } else if kind == 3u {
        alpha *= 0.75;
    }

    return vec4<f32>(in.color.rgb, alpha);
}
