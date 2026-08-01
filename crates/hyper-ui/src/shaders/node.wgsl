struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) border_radius: f32,
    @location(5) border_width: f32,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) border_radius: f32,
    @location(5) border_width: f32,
};

fn corner_offset(i: u32) -> vec2<f32> {
    switch i {
        case 0u: { return vec2<f32>(0.0, 0.0); }
        case 1u: { return vec2<f32>(1.0, 0.0); }
        case 2u: { return vec2<f32>(0.0, 1.0); }
        default: { return vec2<f32>(1.0, 1.0); }
    }
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let corner = corner_offset(in.vertex_index % 4u);
    // Expand slightly for AA fringe
    let pad = 1.0;
    let local = corner * (in.size + vec2<f32>(pad * 2.0, pad * 2.0)) - vec2<f32>(pad, pad);
    let screen = in.position + local;
    let ndc = vec2<f32>(
        (screen.x / globals.screen_size.x) * 2.0 - 1.0,
        1.0 - (screen.y / globals.screen_size.y) * 2.0,
    );
    out.clip_position = vec4<f32>(ndc, 0.0, 1.0);
    out.local = local;
    out.size = in.size;
    out.color = in.color;
    out.border_color = in.border_color;
    out.border_radius = in.border_radius;
    out.border_width = in.border_width;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = min(in.border_radius, min(half.x, half.y));
    let dist = sd_rounded_box(p, half, radius);
    let aa = fwidth(dist) * 0.75;
    let fill_a = 1.0 - smoothstep(-aa, aa, dist);

    var color = in.color;
    if in.border_width > 0.0 {
        let inner = dist + in.border_width;
        let border_mask = smoothstep(-aa, aa, inner) * fill_a;
        color = mix(in.color, in.border_color, border_mask);
    }
    color.a *= fill_a;
    if color.a < 0.001 {
        discard;
    }
    return color;
}
