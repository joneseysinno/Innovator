pub mod axial;
pub mod custom;
pub mod flexure;
pub mod reinforcement;
pub mod shear;
pub mod slenderness;

use super::check_result::CheckResult;
use hypernode::Node;

/// Run all Phase 4 checks for a wall node.
pub fn all_checks(wall: &Node) -> Vec<CheckResult> {
    let mut out = Vec::new();
    out.push(axial::check(wall));
    out.push(shear::check(wall));
    out.push(flexure::check(wall));
    out.push(slenderness::check(wall));
    out.push(reinforcement::check(wall));
    out.extend(custom::checks(wall));
    out
}
