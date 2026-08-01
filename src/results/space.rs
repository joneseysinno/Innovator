use infinite_db::infinitedb_core::address::SpaceId;
use infinite_db::infinitedb_core::space::SpaceConfig;
use infinite_db::InfiniteDb;

/// Spatial space for Results HyperNodes.
pub const RESULTS_SPACE: SpaceId = SpaceId(2);

/// Register the results space if missing.
pub fn ensure_results_space(db: &mut InfiniteDb) {
    let _ = db.register_space(SpaceConfig::new(RESULTS_SPACE, "results", 2));
}
