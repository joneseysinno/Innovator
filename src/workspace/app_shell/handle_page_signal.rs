use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::workspace::analysis::templates::empty_page_ios;
use crate::workspace::analysis::PageSignal;
use crate::workspace::instance::WorkspaceInstance;

pub fn handle_page_signal(shell: &mut AppShell, signal: PageSignal) {
    let active_id = shell.active_id;
    let Some(idx) = shell.workspaces.iter().position(|w| w.id() == active_id) else {
        return;
    };
    let WorkspaceInstance::Analysis(ws) = &mut shell.workspaces[idx] else {
        return;
    };

    let mut rebuild = false;

    match signal {
        PageSignal::Split {
            seam_id,
            direction,
            side,
        } => {
            let new_id = ws.alloc_page_id();
            if ws
                .page_tree
                .split_at_seam(seam_id.0, side, direction, new_id)
                .is_some()
            {
                ws.page_ios.insert(new_id, empty_page_ios());
                rebuild = true;
            }
        }
        PageSignal::SplitPage { page_id, direction } => {
            let new_id = ws.alloc_page_id();
            if ws.page_tree.split_page(page_id, direction, new_id).is_some() {
                ws.page_ios.insert(new_id, empty_page_ios());
                rebuild = true;
            }
        }
        PageSignal::Merge { seam_id, keep } => {
            if let Some(retired) = ws.page_tree.merge(seam_id, keep) {
                for id in retired {
                    ws.page_ios.remove(&id);
                }
                rebuild = true;
            }
        }
        PageSignal::ResetRatio { seam_id } => {
            ws.page_tree.reset_ratio(seam_id);
            rebuild = true;
        }
        PageSignal::ScrollToPod {
            page_id,
            pod_leaf_id,
        } => {
            // Pod rects are absolute today — refresh layouts. Future: scroll offset.
            let _ = (page_id, pod_leaf_id);
            rebuild = true;
        }
    }

    shell.pending_context_menu = None;
    shell.context_menu_triggers.clear();

    if rebuild {
        let mut renderer = match shell.renderer.take() {
            Some(r) => r,
            None => return,
        };
        rebuild_active(shell, &mut renderer);
        shell.renderer = Some(renderer);
    }
}
