use super::PodTree;

impl PodTree {
    pub fn set_ratio(&mut self, seam_index: usize, ratio: f32) {
        let mut idx = 0;
        self.set_ratio_inner(seam_index, ratio.clamp(0.1, 0.9), &mut idx);
    }
}
