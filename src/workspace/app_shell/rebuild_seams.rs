use crate::workspace::analysis::AnalysisWorkspace;
use hyper_ui::{HyperRenderer, Rect};

/// Two-pass seam rebuild: page seams, then pod seams per page content rect.
pub fn rebuild_seams(
    ws: &AnalysisWorkspace,
    pages_area: Rect,
    renderer: &mut HyperRenderer,
) {
    renderer
        .ui
        .page_seams
        .rebuild_from_page_tree(&ws.page_tree, pages_area);

    renderer.ui.pod_seams.clear();
    for (page_id, page_rect) in ws.page_tree.leaf_rects(pages_area) {
        let Some(page) = ws.page_tree.find(page_id) else {
            continue;
        };
        let content_rect = page.content_rect(page_rect);
        renderer
            .ui
            .pod_seams
            .append_from_pods(page_id, &page.pod_tree, content_rect);
    }
}

/// Clear both seam populations (non-analysis workspaces).
pub fn clear_seams(renderer: &mut HyperRenderer) {
    renderer.ui.page_seams.clear();
    renderer.ui.pod_seams.clear();
}
