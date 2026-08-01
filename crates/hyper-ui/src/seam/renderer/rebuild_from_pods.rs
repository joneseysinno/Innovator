use crate::geom::Rect;
use crate::seam::{rebuild_seams, PodTree};

use super::SeamRenderer;

impl SeamRenderer {
    pub fn rebuild_from_pods(&mut self, pods: &PodTree, area: Rect) {
        self.seams.clear();
        rebuild_seams(pods, area, &mut self.seams);
    }
}
