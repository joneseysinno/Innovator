use crate::geom::Rect;
use crate::page::PageTree;
use crate::seam::rebuild_page_seams;

use super::SeamRenderer;

impl SeamRenderer {
    pub fn rebuild_from_page_tree(&mut self, pages: &PageTree, area: Rect) {
        self.seams.clear();
        rebuild_page_seams(pages, area, &mut self.seams);
    }
}
