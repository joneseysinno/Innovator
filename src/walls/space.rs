use infinite_db::infinitedb_core::address::SpaceId;
use infinite_db::infinitedb_core::space::SpaceConfig;
use infinite_db::InfiniteDb;

/// Spatial space for wall HyperNodes.
pub const WALLS_SPACE: SpaceId = SpaceId(1);

/// Register the walls space if missing (idempotent for re-open).
pub fn ensure_walls_space(db: &mut InfiniteDb) {
    if db.register_space(SpaceConfig::new(WALLS_SPACE, "walls", 2)).is_err() {
        // Already registered on a prior open — fine.
    }
}
