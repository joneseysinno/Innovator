use crate::domains::structural::StructuralWorkspace;
use hyper_ui::{HyperRenderer, Rect};

/// Rebuild page seams; pod dividers are rebuilt from each page's PodList layout.
pub fn rebuild_seams(
    ws: &StructuralWorkspace,
    pages_area: Rect,
    renderer: &mut HyperRenderer,
) {
    renderer
        .ui
        .page_seams
        .rebuild_from_page_tree(&ws.page_tree, pages_area);

    renderer.ui.pod_dividers.clear();
    for (page_id, page_rect) in ws.page_tree.leaf_rects(pages_area) {
        let Some(page) = ws.page_tree.find(page_id) else {
            continue;
        };
        let content_rect = page.content_rect(page_rect);
        let layout = page.pods.layout(content_rect);
        renderer
            .ui
            .pod_dividers
            .append(&layout, page.pods.gap, content_rect.size.y);
    }
}

pub fn clear_seams(renderer: &mut HyperRenderer) {
    renderer.ui.page_seams.clear();
    renderer.ui.pod_dividers.clear();
}
