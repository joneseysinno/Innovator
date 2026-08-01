/// CPU-side node data ready for camera transform + upload.
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub world_pos: [f64; 2],
    pub size_world: [f32; 2],
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
    pub selected: bool,
}
