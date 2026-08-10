//! Layout cascade for placeholder page trees.

use super::PlaceholderWorkspace;
use hyper_ui::{FocusPath, PageId, PageNode, PageTree, ResolveReport, Viewport};

impl PlaceholderWorkspace {
    pub fn focus_path(&self) -> FocusPath {
        FocusPath::new(vec![PageNode::container_id(self.focused_page)])
    }

    pub fn layout_pages(
        &mut self,
        pages_area: hyper_ui::Rect,
        app_focus: &FocusPath,
        viewport: &Viewport,
    ) -> (Vec<(PageId, hyper_ui::Rect)>, ResolveReport) {
        if let Some(page_id) = page_id_on_focus(app_focus, &self.page_tree) {
            self.focused_page = page_id;
        }
        let focus = self.focus_path();
        let (rects, report) =
            self.page_tree
                .layout(pages_area, &focus, &self.page_overrides, viewport);
        if let Some((id, _)) = rects.first() {
            if self
                .page_tree
                .find(self.focused_page)
                .map(|p| p.state.resolved() != hyper_ui::Visibility::Shown)
                .unwrap_or(true)
            {
                self.focused_page = *id;
            }
        }
        (rects, report)
    }
}

fn page_id_on_focus(focus: &FocusPath, tree: &PageTree) -> Option<PageId> {
    for id in &focus.chain {
        for leaf in tree.leaves() {
            if PageNode::container_id(leaf.id) == *id {
                return Some(leaf.id);
            }
        }
    }
    None
}
