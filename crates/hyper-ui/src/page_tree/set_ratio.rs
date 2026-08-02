use super::{PageSeamId, PageTree};

impl PageTree {
    pub fn set_ratio(&mut self, seam_id: PageSeamId, ratio: f32) {
        let mut idx = 0u32;
        self.set_ratio_inner(seam_id.0, ratio.clamp(0.1, 0.9), &mut idx);
    }

    pub fn reset_ratio(&mut self, seam_id: PageSeamId) {
        self.set_ratio(seam_id, 0.5);
    }

    /// Index-based API matching [`crate::seam::PodTree::set_ratio`].
    pub fn set_ratio_index(&mut self, seam_index: usize, ratio: f32) {
        self.set_ratio(PageSeamId(seam_index as u32), ratio);
    }

    pub fn reset_ratio_index(&mut self, seam_index: usize) {
        self.reset_ratio(PageSeamId(seam_index as u32));
    }

    fn set_ratio_inner(&mut self, target: u32, ratio: f32, idx: &mut u32) -> bool {
        match self {
            Self::Leaf(_) => false,
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
