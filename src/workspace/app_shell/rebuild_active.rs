use super::build_tree::build_tree;
use super::layout_areas::layout_areas;
use super::rebuild_seams::rebuild_seams;
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::sync_page_layouts::sync_page_layouts;
use super::AppShell;
use hyper_ui::HyperRenderer;

/// Rebuild UI tree and seams for the currently active workspace.
pub fn rebuild_active(shell: &mut AppShell, renderer: &mut HyperRenderer) {
    let root = build_tree(shell);
    let (_tabs, _header, pages) = layout_areas(shell.window_area, shell.has_header);
    shell.pages_area = pages;

    renderer.ui.set_tree(root);

    if let Some(pod) = shell.active().and_then(|a| a.pod_tree()).cloned() {
        rebuild_seams(&pod, shell.pages_area, renderer);
    } else {
        // Empty workspace — clear seams
        renderer.ui.pods = hyper_ui::PodTree::Leaf { id: 0 };
        renderer.ui.seams.rebuild_from_pods(&renderer.ui.pods, shell.pages_area);
    }

    renderer.ui.layout(shell.window_area);
    if let Some(tree_root) = renderer.ui.tree.root.as_mut() {
        sync_chrome_layouts(tree_root, shell.window_area, shell.has_header);
        if let Some(pod) = shell.active().and_then(|a| a.pod_tree()) {
            let leaves = pod.leaf_rects(shell.pages_area);
            sync_page_layouts(tree_root, &leaves);
        }
    }
}
