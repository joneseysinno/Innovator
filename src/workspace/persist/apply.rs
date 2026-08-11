//! Restore PersistedSession → live workspaces (seeds only when no save).

use super::types::*;
use crate::domains::placeholder::{PlaceholderWorkspace, StubIoMap};
use crate::domains::structural::{IoKind, StructuralWorkspace};
use crate::workspace::seed;
use crate::workspace::workspace::{Workspace, WorkspaceBody};
use hyper_ui::container::{ContainerId, ContainerState, Extent, Visibility};
use hyper_ui::{
    Overrides, PageId, PageNode, PageTree, Pod, PodId, PodList, SeamDirection, SizeClass,
};
use infinite_db::InfiniteDb;

pub fn restore_workspaces(
    session: &PersistedSession,
    db: &mut InfiniteDb,
) -> (Vec<Workspace>, u64) {
    let workspaces = session
        .workspaces
        .iter()
        .map(|pw| restore_workspace(pw, db))
        .collect();
    (workspaces, session.next_workspace_id)
}

fn restore_workspace(pw: &PersistedWorkspace, db: &mut InfiniteDb) -> Workspace {
    let open_id = intern_open_id(&pw.open_id);

    let mut body = match open_id {
        "home" => WorkspaceBody::Home(crate::domains::home::HomeWorkspace::new()),
        "structural_analysis" => WorkspaceBody::Structural(StructuralWorkspace::new(db)),
        _ => {
            // Prefer seed skeleton for stub labels, then overwrite tree from save.
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == open_id)
                .unwrap_or(&seed::PROJECT_MANAGEMENT);
            WorkspaceBody::Placeholder(PlaceholderWorkspace::from_seed(seed))
        }
    };

    match &mut body {
        WorkspaceBody::Structural(s) => {
            if let Some(tree) = &pw.page_tree {
                s.page_tree = restore_page_tree(tree);
            }
            s.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                s.focused_page = PageId(fp);
            }
            if let Some(ios) = &pw.page_ios {
                s.page_ios = restore_page_ios(ios);
            }
            if let Some(n) = pw.next_page_id {
                s.next_page_id = n;
            }
        }
        WorkspaceBody::Placeholder(p) => {
            if let Some(tree) = &pw.page_tree {
                p.page_tree = restore_page_tree(tree);
            }
            p.page_overrides = restore_overrides(&pw.page_overrides);
            if let Some(fp) = pw.focused_page {
                p.focused_page = PageId(fp);
            }
            if let Some(stubs) = &pw.stub_ios {
                p.stub_ios = restore_stub_ios(stubs);
            }
        }
        WorkspaceBody::Home(_) => {}
    }

    Workspace {
        state: restore_container(&pw.state),
        open_id,
        body,
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
    match tree {
        PersistedPageTree::Leaf(node) => PageTree::Leaf(restore_page_node(node)),
        PersistedPageTree::Split {
            direction,
            first,
            second,
        } => PageTree::Split {
            direction: restore_seam(*direction),
            first: Box::new(restore_page_tree(first)),
            second: Box::new(restore_page_tree(second)),
        },
    }
}

fn restore_seam(d: PersistedSeamDirection) -> SeamDirection {
    match d {
        PersistedSeamDirection::Vertical => SeamDirection::Vertical,
        PersistedSeamDirection::Horizontal => SeamDirection::Horizontal,
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
    out.scroll_offset = 0.0;
    out
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

fn restore_page_ios(
    entries: &[(u32, Vec<(u32, String)>)],
) -> std::collections::HashMap<PageId, Vec<(PodId, IoKind)>> {
    entries
        .iter()
        .map(|(page, ios)| {
            (
                PageId(*page),
                ios.iter()
                    .map(|(pod, kind)| (PodId(*pod), parse_io_kind(kind)))
                    .collect(),
            )
        })
        .collect()
}

fn restore_stub_ios(entries: &[((u32, u32), Vec<String>)]) -> StubIoMap {
    entries
        .iter()
        .map(|((page, pod), labels)| ((PageId(*page), PodId(*pod)), labels.clone()))
        .collect()
}

fn parse_io_kind(s: &str) -> IoKind {
    match s {
        "WallList" => IoKind::WallList,
        "WallSummary" => IoKind::WallSummary,
        "InputForm" => IoKind::InputForm,
        "WallView" => IoKind::WallView,
        "ResultsTable" => IoKind::ResultsTable,
        "Status" => IoKind::Status,
        _ => IoKind::Empty,
    }
}
