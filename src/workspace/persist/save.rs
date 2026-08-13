//! Capture live shell → PersistedSession and write layout.json.

use super::types::*;
use crate::workspace::app_shell::AppShell;
use crate::workspace::workspace::{Workspace, WorkspaceBody};
use hyper_ui::container::{ContainerState, Extent, Visibility};
use hyper_ui::{Overrides, PageNode, PageTree, Pod, PodList, SizeClass};
use std::path::Path;

pub fn save_layout(path: impl AsRef<Path>, shell: &AppShell) -> std::io::Result<()> {
    let session = capture_session(shell);
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, json)
}

pub fn capture_session(shell: &AppShell) -> PersistedSession {
    PersistedSession {
        version: PERSIST_VERSION,
        next_workspace_id: shell.next_workspace_id,
        workspaces: shell.workspaces.iter().map(capture_workspace).collect(),
    }
}

fn capture_workspace(ws: &Workspace) -> PersistedWorkspace {
    let (page_tree, page_overrides, focused_page, page_templates, pod_templates, next_page_id, stub_ios) =
        match &ws.body {
            WorkspaceBody::Home(h) => (
                Some(capture_page_tree(&h.page_tree)),
                capture_overrides(&h.page_overrides),
                Some(h.focused_page.0),
                None,
                None,
                None,
                None,
            ),
            WorkspaceBody::Structural(s) => (
                Some(capture_page_tree(&s.page_tree)),
                capture_overrides(&s.page_overrides),
                Some(s.focused_page.0),
                Some(capture_page_templates(&s.page_templates)),
                Some(capture_pod_templates(&s.pod_templates)),
                Some(s.next_page_id),
                None,
            ),
            WorkspaceBody::GraphView(g) => (
                Some(capture_page_tree(&g.page_tree)),
                capture_overrides(&g.page_overrides),
                Some(g.focused_page.0),
                None,
                None,
                None,
                None,
            ),
            WorkspaceBody::Placeholder(p) => (
                Some(capture_page_tree(&p.page_tree)),
                capture_overrides(&p.page_overrides),
                Some(p.focused_page.0),
                None,
                None,
                None,
                // Components live in the graph; stub_ios no longer persisted.
                None,
            ),
        };

    PersistedWorkspace {
        open_id: ws.open_id.to_string(),
        state: capture_container(&ws.state),
        focused_page,
        page_tree,
        page_overrides,
        page_templates,
        pod_templates,
        next_page_id,
        stub_ios,
    }
}

fn capture_container(state: &ContainerState) -> PersistedContainer {
    PersistedContainer {
        id: state.id.0,
        label: state.label.clone(),
        icon: state.icon.clone(),
        intent: capture_visibility(state.intent),
        extent: capture_extent(state.extent),
    }
}

fn capture_visibility(v: Visibility) -> PersistedVisibility {
    match v {
        Visibility::Shown => PersistedVisibility::Shown,
        Visibility::Collapsed => PersistedVisibility::Collapsed,
        Visibility::Hidden => PersistedVisibility::Hidden,
    }
}

fn capture_extent(e: Extent) -> PersistedExtent {
    PersistedExtent {
        min: e.min,
        ideal: e.ideal,
        weight: e.weight,
    }
}

pub(crate) fn capture_overrides(o: &Overrides) -> PersistedOverrides {
    let mut entries: Vec<_> = o
        .iter()
        .map(|(id, class, fraction)| PersistedOverrideEntry {
            id: id.0,
            class: capture_size_class(class),
            fraction,
        })
        .collect();
    entries.sort_by_key(|e| (e.id, e.class as u8));

    let mut collapse_entries: Vec<_> = o
        .iter_collapse()
        .map(|(id, class, collapsed)| PersistedCollapseEntry {
            id: id.0,
            class: capture_size_class(class),
            collapsed,
        })
        .collect();
    collapse_entries.sort_by_key(|e| (e.id, e.class as u8));

    PersistedOverrides {
        entries,
        collapse_entries,
    }
}

fn capture_size_class(c: SizeClass) -> PersistedSizeClass {
    match c {
        SizeClass::Compact => PersistedSizeClass::Compact,
        SizeClass::Medium => PersistedSizeClass::Medium,
        SizeClass::Expanded => PersistedSizeClass::Expanded,
        SizeClass::Large => PersistedSizeClass::Large,
    }
}

fn capture_page_tree(tree: &PageTree) -> PersistedPageTree {
    PersistedPageTree {
        pages: tree.pages.iter().map(capture_page_node).collect(),
    }
}

fn capture_page_node(node: &PageNode) -> PersistedPageNode {
    PersistedPageNode {
        id: node.id.0,
        state: capture_container(&node.state),
        pods: capture_pod_list(&node.pods),
    }
}

fn capture_pod_list(list: &PodList) -> PersistedPodList {
    PersistedPodList {
        pods: list.pods.iter().map(capture_pod).collect(),
        gap: list.gap,
        overrides: capture_overrides(&list.overrides),
    }
}

fn capture_pod(pod: &Pod) -> PersistedPod {
    PersistedPod {
        id: pod.id.0,
        state: capture_container(&pod.state),
        title: pod.title.clone(),
        min_height: pod.min_height,
        height: pod.height,
        nav_icon: pod.nav_icon.clone(),
    }
}

fn capture_page_templates(
    map: &std::collections::HashMap<hyper_ui::PageId, hyper_ui::TemplateId>,
) -> Vec<(u32, String)> {
    let mut out: Vec<_> = map
        .iter()
        .map(|(page, template)| (page.0, template.as_str().to_string()))
        .collect();
    out.sort_by_key(|(id, _)| *id);
    out
}

fn capture_pod_templates(
    map: &std::collections::HashMap<(hyper_ui::PageId, hyper_ui::PodId), hyper_ui::TemplateId>,
) -> Vec<(u32, u32, String)> {
    let mut out: Vec<_> = map
        .iter()
        .map(|((page, pod), template)| (page.0, pod.0, template.as_str().to_string()))
        .collect();
    out.sort_by_key(|(page, pod, _)| (*page, *pod));
    out
}
