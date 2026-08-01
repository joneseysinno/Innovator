use crate::engine::check_result::CheckResult;
use crate::walls::prop_f64;
use hypernode::Node;

/// Simplified φVc vs Vu for concrete walls.
pub fn check(wall: &Node) -> CheckResult {
    let thickness = prop_f64(wall, "thickness", 8.0); // in
    let length_ft = prop_f64(wall, "length", 20.0);
    let fc = prop_f64(wall, "fc", 4000.0);
    let lambda = prop_f64(wall, "lambda", 1.0);
    let vu = prop_f64(wall, "vu", 0.0); // kips

    let lw = length_ft * 12.0; // in
    let d = 0.8 * lw; // effective depth approx
    // Vc = 2 λ √f'c t d  (lb) — ACI wall concrete shear (simplified)
    let vc = 2.0 * lambda * fc.max(0.0).sqrt() * thickness * d;
    let phi = 0.75;
    let phi_vc = phi * vc / 1000.0; // kips

    CheckResult::structural("Shear (φVc)", vu, phi_vc, "kips")
}
