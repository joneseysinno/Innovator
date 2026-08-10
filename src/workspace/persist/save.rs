//! Capture live shell → PersistedSession and write layout.json.

use super::types::*;
use crate::domains::structural::IoKind;
use crate::workspace::app_shell::AppShell;
use crate::workspace::workspace::{Workspace, WorkspaceBody};
use hyper_ui::container::{ContainerState, Extent, Visibility};
use hyper_ui::{
    Overrides, PageNode, PageTree, Pod, PodList, SeamDirection, SizeClass,
};
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
    let (page_tree, page_overrides, focused_page, page_ios, next_page_id, stub_ios) =
        match &ws.body {
            WorkspaceBody::Home(_) => (None, PersistedOverrides { entries: vec![] }, None, None, None, None),
            WorkspaceBody::Structural(s) => (
                Some(capture_page_tree(&s.page_tree)),
                capture_overrides(&s.page_overrides),
                Some(s.focused_page.0),
                Some(capture_page_ios(&s.page_ios)),
                Some(s.next_page_id),
                None,
            ),
            WorkspaceBody::Placeholder(p) => (
                Some(capture_page_tree(&p.page_tree)),
                capture_overrides(&p.page_overrides),
                Some(p.focused_page.0),
                None,
                None,
                Some(capture_stub_ios(&p.stub_ios)),
            ),
        };

    PersistedWorkspace {
        open_id: ws.open_id.to_string(),
        state: capture_container(&ws.state),
        focused_page,
        page_tree,
        page_overrides,
        page_ios,
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
    PersistedOverrides { entries }
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
    match tree {
        PageTree::Leaf(node) => PersistedPageTree::Leaf(capture_page_node(node)),
        PageTree::Split {
            direction,
            first,
            second,
        } => PersistedPageTree::Split {
            direction: capture_seam(*direction),
            first: Box::new(capture_page_tree(first)),
            second: Box::new(capture_page_tree(second)),
        },
    }
}

fn capture_seam(d: SeamDirection) -> PersistedSeamDirection {
    match d {
        SeamDirection::Vertical => PersistedSeamDirection::Vertical,
        SeamDirection::Horizontal => PersistedSeamDirection::Horizontal,
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
    }
}

fn capture_page_ios(
    map: &std::collections::HashMap<hyper_ui::PageId, Vec<(hyper_ui::PodId, IoKind)>>,
) -> Vec<(u32, Vec<(u32, String)>)> {
    let mut out: Vec<_> = map
        .iter()
        .map(|(page, ios)| {
            (
                page.0,
                ios.iter()
                    .map(|(pod, kind)| (pod.0, io_kind_name(*kind).to_string()))
                    .collect(),
            )
        })
        .collect();
    out.sort_by_key(|(id, _)| *id);
    out
}

fn capture_stub_ios(
    map: &crate::domains::placeholder::StubIoMap,
) -> Vec<((u32, u32), Vec<String>)> {
    let mut out: Vec<_> = map
        .iter()
        .map(|((page, pod), labels)| ((page.0, pod.0), labels.clone()))
        .collect();
    out.sort_by_key(|((p, d), _)| (*p, *d));
    out
}

fn io_kind_name(kind: IoKind) -> &'static str {
    match kind {
        IoKind::WallList => "WallList",
        IoKind::WallSummary => "WallSummary",
        IoKind::InputForm => "InputForm",
        IoKind::WallView => "WallView",
        IoKind::ResultsTable => "ResultsTable",
        IoKind::Status => "Status",
        IoKind::Empty => "Empty",
    }
}
