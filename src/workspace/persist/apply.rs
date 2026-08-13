//! Restore PersistedSession → live workspaces (seeds only when no save).

use super::types::*;
use crate::domains::graph_view::GraphViewWorkspace;
use crate::domains::placeholder::PlaceholderWorkspace;
use crate::domains::structural::{templates::template_from_str, StructuralWorkspace};
use crate::workspace::graph_containers::{
    dual_write_page_tree, write_components_from_page_seeds, write_pod_components,
};
use crate::workspace::seed;
use crate::workspace::workspace::{Workspace, WorkspaceBody};
use hyper_ui::container::{ContainerId, ContainerState, Extent, Visibility};
use hyper_ui::{Overrides, PageId, PageNode, PageTree, Pod, PodId, PodList, SizeClass};
use hypernode::{Graph, PropValue};
use infinite_db::InfiniteDb;

pub fn restore_workspaces(
    session: &PersistedSession,
    db: &mut InfiniteDb,
    graph: &mut Graph,
) -> (Vec<Workspace>, u64) {
    let workspaces = session
        .workspaces
        .iter()
        .map(|pw| restore_workspace(pw, db, graph))
        .collect();
    (workspaces, session.next_workspace_id)
}

fn restore_workspace(pw: &PersistedWorkspace, db: &mut InfiniteDb, graph: &mut Graph) -> Workspace {
    let open_id = intern_open_id(&pw.open_id);

    let mut body = match open_id {
        "home" => WorkspaceBody::Home(crate::domains::home::HomeWorkspace::from_seed(graph)),
        "structural_analysis" => WorkspaceBody::Structural(StructuralWorkspace::new(db, graph)),
        "devtools_graph" => {
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == open_id)
                .unwrap_or(&seed::DEVTOOLS_GRAPH);
            WorkspaceBody::GraphView(GraphViewWorkspace::from_seed(seed, graph))
        }
        _ => {
            // Prefer seed skeleton for stub labels, then overwrite tree from save.
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == open_id)
                .unwrap_or(&seed::PROJECT_MANAGEMENT);
            WorkspaceBody::Placeholder(PlaceholderWorkspace::from_seed(seed, graph))
        }
    };

    match &mut body {
        WorkspaceBody::Structural(s) => {
            if let Some(tree) = &pw.page_tree {
                s.page_tree = restore_page_tree(tree);
                dual_write_page_tree(graph, s.node_id, &mut s.page_tree);
            }
            write_components_from_page_seeds(graph, &s.page_tree, seed::STRUCTURAL.pages);
            s.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                s.focused_page = PageId(fp);
            }
            if let Some(templates) = &pw.page_templates {
                s.page_templates = restore_page_templates(templates);
            }
            if let Some(templates) = &pw.pod_templates {
                s.pod_templates = restore_pod_templates(templates);
            }
            for page in &s.page_tree.pages {
                if let Some(template) = s.page_templates.get(&page.id) {
                    graph.nodes.get_mut(&page.node_id).expect("page UIView").props.insert(
                        "template_id".into(),
                        PropValue::Text(template.as_str().into()),
                    );
                }
                for pod in &page.pods.pods {
                    if let Some(template) = s.pod_templates.get(&(page.id, pod.id)) {
                        graph.nodes.get_mut(&pod.node_id).expect("pod UIView").props.insert(
                            "template_id".into(),
                            PropValue::Text(template.as_str().into()),
                        );
                    }
                }
            }
            if let Some(n) = pw.next_page_id {
                s.next_page_id = n;
            }
        }
        WorkspaceBody::GraphView(g) => {
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == open_id)
                .unwrap_or(&seed::DEVTOOLS_GRAPH);
            if let Some(tree) = &pw.page_tree {
                g.page_tree = restore_page_tree(tree);
                dual_write_page_tree(graph, g.node_id, &mut g.page_tree);
            }
            write_components_from_page_seeds(graph, &g.page_tree, seed.pages);
            g.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                g.focused_page = PageId(fp);
            }
        }
        WorkspaceBody::Placeholder(p) => {
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == open_id)
                .unwrap_or(&seed::PROJECT_MANAGEMENT);
            if let Some(tree) = &pw.page_tree {
                p.page_tree = restore_page_tree(tree);
                dual_write_page_tree(graph, p.node_id, &mut p.page_tree);
            }
            // Prefer legacy stub_ios labels when present; else rebuild from seed.
            if let Some(stubs) = &pw.stub_ios {
                for ((page_id, pod_id), labels) in stubs {
                    if let Some(page) = p.page_tree.pages.iter().find(|pg| pg.id.0 == *page_id) {
                        if let Some(pod) = page.pods.pods.iter().find(|pd| pd.id.0 == *pod_id) {
                            let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
                            write_pod_components(graph, pod.node_id, &refs);
                        }
                    }
                }
            } else {
                write_components_from_page_seeds(graph, &p.page_tree, seed.pages);
            }
            p.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                p.focused_page = PageId(fp);
            }
        }
        WorkspaceBody::Home(h) => {
            if let Some(tree) = &pw.page_tree {
                h.page_tree = restore_page_tree(tree);
                dual_write_page_tree(graph, h.node_id, &mut h.page_tree);
            }
            write_components_from_page_seeds(graph, &h.page_tree, seed::HOME.pages);
            h.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                h.focused_page = PageId(fp);
            }
        }
    }

    let node_id = match &body {
        WorkspaceBody::Home(h) => h.node_id,
        WorkspaceBody::Structural(s) => s.node_id,
        WorkspaceBody::GraphView(g) => g.node_id,
        WorkspaceBody::Placeholder(p) => p.node_id,
    };

    Workspace {
        state: restore_container(&pw.state),
        open_id,
        body,
        node_id,
    }
}

fn intern_open_id(s: &str) -> &'static str {
    seed::ALL
        .iter()
        .find(|seed| seed.open_id == s)
        .map(|seed| seed.open_id)
        .unwrap_or(seed::PROJECT_MANAGEMENT.open_id)
}

fn restore_container(p: &PersistedContainer) -> ContainerState {
    ContainerState::new(
        ContainerId(p.id),
        p.label.clone(),
        p.icon.clone(),
        restore_visibility(p.intent),
        restore_extent(p.extent),
    )
}

fn restore_visibility(v: PersistedVisibility) -> Visibility {
    match v {
        PersistedVisibility::Shown => Visibility::Shown,
        PersistedVisibility::Collapsed => Visibility::Collapsed,
        PersistedVisibility::Hidden => Visibility::Hidden,
    }
}

fn restore_extent(e: PersistedExtent) -> Extent {
    Extent::new(e.min, e.ideal, e.weight)
}

pub(crate) fn restore_overrides(p: &PersistedOverrides) -> Overrides {
    Overrides::from_entries(p.entries.iter().map(|e| {
        (
            ContainerId(e.id),
            restore_size_class(e.class),
            e.fraction,
        )
    }))
    .merge_collapse_entries(p.collapse_entries.iter().map(|e| {
        (
            ContainerId(e.id),
            restore_size_class(e.class),
            e.collapsed,
        )
    }))
}

fn restore_size_class(c: PersistedSizeClass) -> SizeClass {
    match c {
        PersistedSizeClass::Compact => SizeClass::Compact,
        PersistedSizeClass::Medium => SizeClass::Medium,
        PersistedSizeClass::Expanded => SizeClass::Expanded,
        PersistedSizeClass::Large => SizeClass::Large,
    }
}

fn restore_page_tree(tree: &PersistedPageTree) -> PageTree {
    PageTree {
        pages: tree.pages.iter().map(restore_page_node).collect(),
    }
}

fn restore_page_node(node: &PersistedPageNode) -> PageNode {
    let mut page = PageNode::new(PageId(node.id), restore_pod_list(&node.pods));
    page.state = restore_container(&node.state);
    page
}

fn restore_pod_list(list: &PersistedPodList) -> PodList {
    let mut out = PodList::new(list.pods.iter().map(restore_pod).collect());
    out.gap = list.gap;
    out.overrides = restore_overrides(&list.overrides);
    migrate_legacy_pod_collapse(&mut out);
    out.scroll_offset = 0.0;
    out
}

/// Pre-Phase-4 saves stored collapse in pod intent globally; map to all size classes.
fn migrate_legacy_pod_collapse(list: &mut PodList) {
    for pod in &list.pods {
        if !matches!(pod.state.intent, Visibility::Collapsed) {
            continue;
        }
        let id = Pod::container_id(pod.id);
        for class in [
            SizeClass::Compact,
            SizeClass::Medium,
            SizeClass::Expanded,
            SizeClass::Large,
        ] {
            if list.overrides.get_collapse(id, class).is_none() {
                list.overrides.set_collapse(id, class, true);
            }
        }
    }
}

fn restore_pod(p: &PersistedPod) -> Pod {
    let mut pod = Pod::new(PodId(p.id), p.title.clone())
        .with_min_height(p.min_height)
        .with_height(p.height);
    if let Some(icon) = &p.nav_icon {
        pod = pod.with_nav_icon(icon.clone());
    }
    pod.state = restore_container(&p.state);
    // Align collapsed flag with intent.
    pod.collapsed = matches!(pod.state.intent, Visibility::Collapsed);
    pod
}

fn restore_page_templates(
    entries: &[(u32, String)],
) -> std::collections::HashMap<PageId, hyper_ui::TemplateId> {
    entries
        .iter()
        .map(|(page, template)| (PageId(*page), template_from_str(template)))
        .collect()
}

fn restore_pod_templates(
    entries: &[(u32, u32, String)],
) -> std::collections::HashMap<(PageId, PodId), hyper_ui::TemplateId> {
    entries
        .iter()
        .map(|(page, pod, template)| ((PageId(*page), PodId(*pod)), template_from_str(template)))
        .collect()
}
