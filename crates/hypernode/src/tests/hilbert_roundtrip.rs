use crate::{hilbert_decode_2d, hilbert_encode_2d};

#[test]
fn hilbert_roundtrip() {
    for order in [2u32, 4, 8] {
        let n = 1u32 << order;
        for x in 0..n.min(16) {
            for y in 0..n.min(16) {
                let d = hilbert_encode_2d(x, y, order);
                let (dx, dy) = hilbert_decode_2d(d, order);
                assert_eq!((x, y), (dx, dy));
            }
        }
    }
}
