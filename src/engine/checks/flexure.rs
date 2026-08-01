use crate::engine::check_result::CheckResult;
use crate::walls::prop_f64;
use hypernode::Node;

/// Simplified φMn vs Mu for a singly reinforced wall section strip.
pub fn check(wall: &Node) -> CheckResult {
    let thickness = prop_f64(wall, "thickness", 8.0); // in
    let cover = prop_f64(wall, "clear_cover", 0.75);
    let fc = prop_f64(wall, "fc", 4000.0);
    let fy = prop_f64(wall, "fy", 60000.0);
    let bar = prop_f64(wall, "vert_bar_size", 5.0);
    let spacing = prop_f64(wall, "vert_spacing", 12.0).max(1.0);
    let mu = prop_f64(wall, "mu", 0.0); // kip-ft

    let db = bar / 8.0;
    let ab = std::f64::consts::PI * (db * 0.5).powi(2);
    // Per-foot strip
    let as_ft = ab * (12.0 / spacing); // in²/ft
    let d = (thickness - cover - db * 0.5).max(1.0);
    let a = (as_ft * fy) / (0.85 * fc * 12.0); // in, b=12
    let mn = as_ft * fy * (d - a / 2.0) / 12_000.0; // kip-ft per ft width
    let phi = 0.90;
    let phi_mn = phi * mn;

    CheckResult::structural("Flexure (φMn)", mu, phi_mn, "kip-ft")
}
