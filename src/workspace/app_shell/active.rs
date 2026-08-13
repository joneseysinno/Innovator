use super::AppShell;
use crate::workspace::workspace::Workspace;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::container::Visibility;
use hyper_ui::{FocusPath, PageNode};

impl AppShell {
    /// The Shown workspace.
    pub fn active(&self) -> Option<&Workspace> {
        self.workspaces
            .iter()
            .find(|w| w.state.intent == Visibility::Shown)
    }

    pub fn active_mut(&mut self) -> Option<&mut Workspace> {
        self.workspaces
            .iter_mut()
            .find(|w| w.state.intent == Visibility::Shown)
    }

    /// Hide every workspace, then show `id` and update focus.
    pub fn set_active(&mut self, id: WorkspaceId) -> bool {
        if !self.workspaces.iter().any(|w| w.id() == id) {
            return false;
        }
        for w in &mut self.workspaces {
            if w.id() == id {
                w.show();
            } else {
                w.hide();
            }
        }
        if let Some(ws) = self.workspaces.iter().find(|w| w.id() == id) {
            let mut chain = vec![ws.state.id];
            if let Some(structural) = ws.structural() {
                chain.push(PageNode::container_id(structural.focused_page));
            } else if let Some(gv) = ws.graph_view() {
                chain.push(PageNode::container_id(gv.focused_page));
            } else if let Some(ph) = ws.placeholder() {
                chain.push(PageNode::container_id(ph.focused_page));
            }
            self.focus = FocusPath::new(chain);
        }
        true
    }
}
