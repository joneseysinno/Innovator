use super::build_tree::build_tree;
use super::layout_areas::layout_areas;
use super::rebuild_seams::{clear_seams, rebuild_seams};
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::sync_from_page_tree::sync_from_page_tree;
use super::AppShell;
use hyper_ui::HyperRenderer;

/// Rebuild UI tree and seams for the currently active workspace.
pub fn rebuild_active(shell: &mut AppShell, renderer: &mut HyperRenderer) {
    shell.has_header = shell.active().and_then(|a| a.header()).is_some();
    let layout = shell.layout_area();
    let (_tabs, _header, pages) = layout_areas(layout, shell.has_header);
    shell.pages_area = pages;
    let pages_area = shell.pages_area;

    let viewport = shell.resolve_viewport();
    let focus = shell.focus.clone();
    if let Some(ws) = shell.active_mut().and_then(|a| a.structural_mut()) {
        let (_rects, report) = ws.layout_pages(pages_area, &focus, &viewport);
        shell.last_report = report;
    } else if let Some(ws) = shell.active_mut().and_then(|a| a.placeholder_mut()) {
        let (_rects, report) = ws.layout_pages(pages_area, &focus, &viewport);
        shell.last_report = report;
    }

    let root = build_tree(shell);
    renderer.ui.set_tree(root);

    match shell.active().and_then(|a| a.page_tree()) {
        Some(tree) => rebuild_seams(tree, shell.pages_area, renderer),
        None => clear_seams(renderer),
    }

    let layout = shell.layout_area();
    renderer.ui.layout(layout);
    if let Some(tree_root) = renderer.ui.tree.root.as_mut() {
        sync_chrome_layouts(tree_root, layout, shell.has_header);
    }
    let pages_area = shell.pages_area;
    if let Some(tree) = shell.active_mut().and_then(|a| a.page_tree_mut()) {
        if let Some(tree_root) = renderer.ui.tree.root.as_mut() {
            sync_from_page_tree(tree_root, tree, pages_area);
        }
    }
}
