use super::build_tree::build_tree;
use super::layout_areas::layout_areas;
use super::rebuild_seams::{clear_seams, rebuild_seams};
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::sync_from_page_tree::sync_from_page_tree;
use super::AppShell;
use crate::workspace::instance::WorkspaceInstance;
use hyper_ui::HyperRenderer;

/// Rebuild UI tree and seams for the currently active workspace.
pub fn rebuild_active(shell: &mut AppShell, renderer: &mut HyperRenderer) {
    let root = build_tree(shell);
    let (_tabs, _header, pages) = layout_areas(shell.window_area, shell.has_header);
    shell.pages_area = pages;

    renderer.ui.set_tree(root);

    match shell.active() {
        Some(WorkspaceInstance::Analysis(ws)) => {
            rebuild_seams(ws, shell.pages_area, renderer);
        }
        _ => clear_seams(renderer),
    }

    renderer.ui.layout(shell.window_area);
    if let Some(tree_root) = renderer.ui.tree.root.as_mut() {
        sync_chrome_layouts(tree_root, shell.window_area, shell.has_header);
        if let Some(WorkspaceInstance::Analysis(ws)) = shell.active() {
            sync_from_page_tree(tree_root, ws, shell.pages_area);
        }
    }
}
