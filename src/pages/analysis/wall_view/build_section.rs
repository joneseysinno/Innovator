use crate::walls::prop_f64;
use hyper_ui::{EdgeKindGpu, InMemoryWorldSpatial, SceneNode, WorldEdge};
use hypernode::Node;

/// Build a 2D wall cross-section (thickness × height) in inches.
pub fn build_section_spatial(node: &Node) -> InMemoryWorldSpatial {
    let thickness = prop_f64(node, "thickness", 8.0).max(1.0);
    let height_ft = prop_f64(node, "height", 12.0).max(1.0);
    let height = height_ft * 12.0;
    let cover = prop_f64(node, "clear_cover", 0.75).clamp(0.25, thickness / 2.0);
    let vert_spacing = prop_f64(node, "vert_spacing", 12.0).max(2.0);
    let bar_num = prop_f64(node, "vert_bar_size", 5.0);
    // Approximate bar diameter (in) from bar number: #n ≈ n/8 in.
    let bar_dia = (bar_num / 8.0).clamp(0.25, 1.5);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Concrete outline (centered at origin).
    nodes.push(SceneNode {
        world_pos: [0.0, 0.0],
        size_world: [thickness as f32, height as f32],
        color: [0.55, 0.55, 0.58, 1.0],
        border_color: [0.85, 0.85, 0.88, 1.0],
        border_radius: 0.05,
        border_width: 2.0,
        selected: false,
    });

    // Cover inset indicator (slightly smaller, darker).
    let inner_w = (thickness - 2.0 * cover).max(0.5);
    let inner_h = (height - 2.0 * cover).max(0.5);
    nodes.push(SceneNode {
        world_pos: [0.0, 0.0],
        size_world: [inner_w as f32, inner_h as f32],
        color: [0.45, 0.46, 0.50, 0.35],
        border_color: [0.70, 0.72, 0.78, 0.8],
        border_radius: 0.02,
        border_width: 1.0,
        selected: false,
    });

    // Vertical rebar along height at left/right cover lines.
    let x_left = -thickness / 2.0 + cover;
    let x_right = thickness / 2.0 - cover;
    let y_min = -height / 2.0 + cover;
    let y_max = height / 2.0 - cover;
    let mut y = y_min;
    while y <= y_max + 0.01 {
        for x in [x_left, x_right] {
            nodes.push(SceneNode {
                world_pos: [x, y],
                size_world: [bar_dia as f32, bar_dia as f32],
                color: [0.75, 0.35, 0.20, 1.0],
                border_color: [0.95, 0.55, 0.30, 1.0],
                border_radius: 0.5,
                border_width: 1.0,
                selected: false,
            });
        }
        y += vert_spacing;
    }

    // Dimension line — thickness (below).
    let dim_y = -height / 2.0 - 2.0;
    edges.push(WorldEdge {
        source: [-thickness / 2.0, dim_y],
        target: [thickness / 2.0, dim_y],
        curvature: 0.0,
        color: [0.55, 0.75, 0.95, 1.0],
        width: 1.5,
        kind: EdgeKindGpu::Binding,
    });
    // Dimension line — height (left).
    let dim_x = -thickness / 2.0 - 2.0;
    edges.push(WorldEdge {
        source: [dim_x, -height / 2.0],
        target: [dim_x, height / 2.0],
        curvature: 0.0,
        color: [0.55, 0.75, 0.95, 1.0],
        width: 1.5,
        kind: EdgeKindGpu::Binding,
    });

    InMemoryWorldSpatial { nodes, edges }
}

/// Empty spatial when no wall is selected.
pub fn empty_section_spatial() -> InMemoryWorldSpatial {
    InMemoryWorldSpatial::default()
}
