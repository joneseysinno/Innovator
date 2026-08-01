use crate::geom::WorldRect;

use super::SceneNode;

/// Cull helper that runs an `infinite-db` bounding-box query then maps records
/// through `map_record`. Hilbert block pruning happens inside `query_bbox`.
pub fn cull_nodes_from_infinite_db<F>(
    db: &mut infinite_db::InfiniteDb,
    space: infinite_db::infinitedb_core::address::SpaceId,
    world: WorldRect,
    grid_origin: [f64; 2],
    cell_size: f64,
    mut map_record: F,
) -> Vec<SceneNode>
where
    F: FnMut(&infinite_db::infinitedb_core::block::Record) -> Option<SceneNode>,
{
    use infinite_db::infinitedb_core::address::DimensionVector;

    let to_cell = |v: f64, axis: usize| -> u32 {
        (((v - grid_origin[axis]) / cell_size).floor() as i64).clamp(0, u32::MAX as i64) as u32
    };

    let min = DimensionVector::new(vec![to_cell(world.min[0], 0), to_cell(world.min[1], 1)]);
    let max = DimensionVector::new(vec![to_cell(world.max[0], 0), to_cell(world.max[1], 1)]);

    match db.query_bbox(space, min, max, None) {
        Ok(records) => records.iter().filter_map(|r| map_record(r)).collect(),
        Err(_) => Vec::new(),
    }
}
