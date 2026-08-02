use crate::geom::Rect;
use crate::page_tree::PageId;
use crate::seam::{rebuild_seams, PodTree};

use super::SeamRenderer;

impl SeamRenderer {
    pub fn rebuild_from_pods(&mut self, pods: &PodTree, area: Rect) {
        self.seams.clear();
        self.pod_owners.clear();
        rebuild_seams(pods, area, &mut self.seams);
    }

    /// Append pod seams for one page into this renderer (multi-page pass).
    pub fn append_from_pods(&mut self, page_id: PageId, pods: &PodTree, area: Rect) {
        let start = self.seams.len();
        rebuild_seams(pods, area, &mut self.seams);
        for local in 0..(self.seams.len() - start) {
            self.pod_owners.push((page_id, local));
        }
    }
}
