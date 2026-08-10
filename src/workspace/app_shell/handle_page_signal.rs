use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::domains::structural::templates::empty_page_ios;
use crate::domains::structural::PageSignal;
use hyper_ui::particles::Particle;
use hyper_ui::Pod;

pub fn handle_page_signal(shell: &mut AppShell, signal: PageSignal) {
    let Some(idx) = shell.workspaces.iter().position(|w| w.is_active()) else {
        return;
    };
    let Some(ws) = shell.workspaces[idx].structural_mut() else {
        return;
    };

    let pages_area = shell.pages_area;
    let mut rebuild = false;
    let mut scroll_to: Option<(hyper_ui::PageId, hyper_ui::PodId)> = None;

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
            ws.reset_page_seam(seam_id.0 as usize, pages_area);
            rebuild = true;
        }
        PageSignal::ScrollToPod { page_id, pod_id } => {
            if let Some(page) = ws.page_tree.find_mut(page_id) {
                page.pods.expand(pod_id);
            }
            scroll_to = Some((page_id, pod_id));
            rebuild = true;
        }
    }

    shell.pending_context_menu = None;
    shell.context_menu_triggers.clear();

    if rebuild {
        shell.persist_layout();
        let mut renderer = match shell.renderer.take() {
            Some(r) => r,
            None => return,
        };
        rebuild_active(shell, &mut renderer);

        if let Some((page_id, pod_id)) = scroll_to {
            if let Some(ws) = shell.workspaces[idx].structural_mut() {
                if let Some(vp_id) = ws.page_viewport_ids.get(&page_id).copied() {
                    let container = Pod::container_id(pod_id);
                    if renderer.ui.tree.scroll_to_container(vp_id, container) {
                        if let Some(Particle::Viewport(vp)) = renderer.ui.tree.find(vp_id) {
                            let offset = vp.offset;
                            if let Some(page) = ws.page_tree.find_mut(page_id) {
                                page.pods.scroll_offset = offset;
                            }
                        }
                    }
                }
            }
        }

        shell.renderer = Some(renderer);
    }
}
