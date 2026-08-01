use crate::engine::check_result::CheckResult;
use crate::walls::prop_f64;
use hypernode::Node;

/// Minimum vertical reinforcement ratio for special structural walls (ρ ≥ 0.0012).
pub fn check(wall: &Node) -> CheckResult {
    let thickness = prop_f64(wall, "thickness", 8.0);
    let bar = prop_f64(wall, "vert_bar_size", 5.0);
    let spacing = prop_f64(wall, "vert_spacing", 12.0).max(1.0);

    let db = bar / 8.0;
    let ab = std::f64::consts::PI * (db * 0.5).powi(2);
    // Two curtains
    let as_ft = 2.0 * ab * (12.0 / spacing);
    let ag_ft = thickness * 12.0;
    let rho = as_ft / ag_ft;
    let min_rho = 0.0012;

    CheckResult::structural("Min vert ρ", min_rho, rho.max(1e-9), "")
}
