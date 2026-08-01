use super::rot::rot;

/// Decode a Hilbert index back to (x, y).
pub fn hilbert_decode_2d(mut d: u64, order: u32) -> (u32, u32) {
    assert!(order > 0 && order <= 16);
    let mut x = 0u32;
    let mut y = 0u32;
    let mut s = 1u32;
    let n = 1u32 << order;
    while s < n {
        let rx = 1u32 & (d / 2) as u32;
        let ry = 1u32 & (d as u32 ^ rx);
        rot(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        d /= 4;
        s *= 2;
    }
    (x, y)
}
