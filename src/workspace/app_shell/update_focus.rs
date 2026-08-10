//! Update [`FocusPath`] from pointer-down inside containers.

use super::AppShell;
use hyper_ui::{PageNode, PageTree, Pod, Vec2};

/// Pointer-down inside a page/pod updates the focus chain. Hover must not call this.
///
/// Returns `true` when the chain changed (caller should rebuild layout).
pub fn update_focus_from_pointer(shell: &mut AppShell, pos: Vec2) -> bool {
    let pages_area = shell.pages_area;
    if !pages_area.contains(pos) {
        return false;
    }

    let Some(ws_idx) = shell.workspaces.iter().position(|w| w.is_active()) else {
        return false;
    };
    let workspace_id = shell.workspaces[ws_idx].state.id;

    let Some(tree) = shell.workspaces[ws_idx].page_tree() else {
        let next = hyper_ui::FocusPath::new(vec![workspace_id]);
        if shell.focus.chain != next.chain {
            shell.focus = next;
            return true;
        }
        return false;
    };

    let Some((page_id, chain)) = focus_chain_at(tree, workspace_id, pages_area, pos) else {
        return false;
    };

    let next = hyper_ui::FocusPath::new(chain);
    if shell.focus.chain == next.chain {
        return false;
    }

    if let Some(ws) = shell.workspaces[ws_idx].structural_mut() {
        ws.focused_page = page_id;
    } else if let Some(ws) = shell.workspaces[ws_idx].placeholder_mut() {
        ws.focused_page = page_id;
    }
    shell.focus = next;
    true
}

fn focus_chain_at(
    tree: &PageTree,
    workspace_id: hyper_ui::ContainerId,
    pages_area: hyper_ui::Rect,
    pos: Vec2,
) -> Option<(hyper_ui::PageId, Vec<hyper_ui::ContainerId>)> {
    let leaf_rects = tree.leaf_rects(pages_area);
    let (page_id, page_rect) = leaf_rects.into_iter().find(|(_, r)| r.contains(pos))?;
    let mut chain = vec![workspace_id, PageNode::container_id(page_id)];
    if let Some(page) = tree.find(page_id) {
        let content = page.content_rect(page_rect);
        let pod_rects = page.pods.layout_rects(content);
        let scroll = page.pods.scroll_offset;
        if let Some((pod_id, _)) = pod_rects.into_iter().find(|(_, r)| {
            let screen =
                hyper_ui::Rect::from_xywh(r.origin.x, r.origin.y - scroll, r.size.x, r.size.y);
            screen.contains(pos)
        }) {
            chain.push(Pod::container_id(pod_id));
        }
    }
    Some((page_id, chain))
}
