use crate::engine::check_result::CheckResult;
use crate::walls::prop_f64;
use hypernode::Node;

/// Simplified φPn (tied wall) vs Pu — ACI 318-style approximation.
pub fn check(wall: &Node) -> CheckResult {
    let thickness = prop_f64(wall, "thickness", 8.0); // in
    let length_ft = prop_f64(wall, "length", 20.0); // ft
    let fc = prop_f64(wall, "fc", 4000.0); // psi
    let fy = prop_f64(wall, "fy", 60000.0); // psi
    let bar = prop_f64(wall, "vert_bar_size", 5.0);
    let spacing = prop_f64(wall, "vert_spacing", 12.0).max(1.0); // in
    let pu = prop_f64(wall, "pu", 0.0); // kips

    let ag = thickness * length_ft * 12.0; // in²
    // Two curtains assumed for special wall; bar area ≈ π(db/2)², db = bar/8.
    let db = bar / 8.0;
    let ab = std::f64::consts::PI * (db * 0.5).powi(2);
    let n_bars = ((length_ft * 12.0) / spacing).floor().max(1.0) * 2.0;
    let ast = n_bars * ab;

    let po = 0.85 * fc * (ag - ast) + fy * ast; // lb
    let phi = 0.65;
    let phi_pn = 0.80 * phi * po / 1000.0; // kips

    CheckResult::structural("Axial (φPn)", pu, phi_pn, "kips")
}
