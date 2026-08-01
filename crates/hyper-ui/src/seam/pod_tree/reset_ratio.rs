use super::PodTree;

impl PodTree {
    pub fn reset_ratio(&mut self, seam_index: usize) {
        self.set_ratio(seam_index, 0.5);
    }
}
