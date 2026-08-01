use hyper_ui::{HyperRenderer, PodTree, Rect};

pub fn rebuild_seams(pod_tree: &PodTree, pages_area: Rect, renderer: &mut HyperRenderer) {
    renderer.ui.pods = pod_tree.clone();
    renderer
        .ui
        .seams
        .rebuild_from_pods(&renderer.ui.pods, pages_area);
}
