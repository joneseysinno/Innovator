/// Quantize a world-space f64 coordinate into a Hilbert grid cell.
pub fn world_to_hilbert_cell(world: f64, origin: f64, cell_size: f64, order: u32) -> u32 {
    let max = (1u32 << order) - 1;
    let cell = ((world - origin) / cell_size).floor() as i64;
    cell.clamp(0, max as i64) as u32
}
