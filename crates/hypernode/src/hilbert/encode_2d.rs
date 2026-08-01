use super::rot::rot;

/// Encode a 2D point into a Hilbert curve index (order bits per axis).
///
/// Useful for spatial range queries and `infinite-db` addressing.
pub fn hilbert_encode_2d(x: u32, y: u32, order: u32) -> u64 {
    assert!(order > 0 && order <= 16);
    let mut x = x;
    let mut y = y;
    let mut d: u64 = 0;
    let mut s = 1u32 << (order - 1);
    while s > 0 {
        let rx = if (x & s) > 0 { 1u32 } else { 0 };
        let ry = if (y & s) > 0 { 1u32 } else { 0 };
        d += (s as u64) * (s as u64) * (((3 * rx) ^ ry) as u64);
        rot(s, &mut x, &mut y, rx, ry);
        s >>= 1;
    }
    d
}
