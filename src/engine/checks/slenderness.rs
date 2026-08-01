use crate::engine::check_result::CheckResult;
use crate::walls::prop_f64;
use hypernode::Node;

/// Special wall h/t limit (simplified ACI guidance: h/t ≤ 25).
pub fn check(wall: &Node) -> CheckResult {
    let height_ft = prop_f64(wall, "height", 12.0);
    let thickness = prop_f64(wall, "thickness", 8.0).max(0.1); // in
    let h_in = height_ft * 12.0;
    let ratio = h_in / thickness;
    let limit = 25.0;
    // Demand = h/t, capacity = limit
    CheckResult::structural("Slenderness (h/t)", ratio, limit, "")
}
