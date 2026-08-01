use super::PodTree;

impl PodTree {
    pub(crate) fn set_ratio_inner(&mut self, target: usize, ratio: f32, idx: &mut usize) -> bool {
        match self {
            Self::Leaf { .. } => false,
            Self::Split {
                ratio: r,
                first,
                second,
                ..
            } => {
                if *idx == target {
                    *r = ratio;
                    return true;
                }
                *idx += 1;
                if first.set_ratio_inner(target, ratio, idx) {
                    return true;
                }
                second.set_ratio_inner(target, ratio, idx)
            }
        }
    }
}
