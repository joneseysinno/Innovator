//! Structural analysis workspace (optional header + page tree of spatial containers).

use super::action::AnalysisAction;
use super::field_builder_draft::{BuilderFieldSlot, FieldBuilderDraft};
use super::kind::AnalysisKind;
use crate::engine::AnalysisOutput;
use crate::pages::analysis::input_form::form_density::FormDensity;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::{
    FocusPath, InMemoryWorldSpatial, Overrides, PageId, PageNode, PageTree, ParticleId, PodId,
    SizeClass as LayoutSizeClass, TemplateId, Viewport,
};
use hypernode::{Node, NodeId};
use std::collections::HashMap;

pub struct StructuralWorkspace {
    /// Action header — present for this workspace kind.
    pub header: Option<WorkspaceHeader>,
    /// Ordered page cache, mirrored by workspace Binding children.
    pub page_tree: PageTree,
    /// Page and pod template assignments mirrored onto UIView node props.
    pub page_templates: HashMap<PageId, TemplateId>,
    pub pod_templates: HashMap<(PageId, PodId), TemplateId>,
    /// Next PageId for split-created pages.
    pub next_page_id: u32,
    /// Which analysis type is active in this workspace.
    pub active_analysis: AnalysisKind,
    pub active_wall: Option<NodeId>,
    /// Sink id → wall NodeId (Navigation WallListIO).
    pub wall_sinks: HashMap<ParticleId, NodeId>,
    /// Trigger id → signal (Navigation "+ New Wall").
    pub nav_triggers: HashMap<ParticleId, WorkspaceSignal>,
    /// Field id → prop key (InputFormIO).
    pub field_props: HashMap<ParticleId, String>,
    /// Field ids that should commit as U8.
    pub u8_fields: HashMap<ParticleId, ()>,
    /// Analysis-page triggers (FieldBuilder, type chips).
    pub analysis_actions: HashMap<ParticleId, AnalysisAction>,
    /// FieldBuilder draft field slots.
    pub builder_slots: HashMap<ParticleId, BuilderFieldSlot>,
    /// Promote trigger → custom prop key.
    pub promote_props: HashMap<ParticleId, String>,
    /// Open inline field builder draft.
    pub field_builder: Option<FieldBuilderDraft>,
    /// Input form label density from pod width.
    pub input_size_class: FormDensity,
    /// Wall view pan/zoom sink.
    pub wall_view_sink: Option<ParticleId>,
    /// Layer A spatial for the active wall section.
    pub wall_spatial: InMemoryWorldSpatial,
    /// Last pointer pos for pan delta.
    pub wall_view_last_pos: Option<hyper_ui::Vec2>,
    pub wall_view_panning: bool,
    /// Latest ResultsNode for the active wall.
    pub last_results: Option<Node>,
    /// Parsed engine summary for StatusIO.
    pub last_analysis: Option<AnalysisOutput>,
    /// Results page triggers (Export PDF).
    pub results_triggers: HashMap<ParticleId, WorkspaceSignal>,
    /// Icon rail triggers: ParticleId → (page_id, pod_id).
    pub icon_rail_triggers: HashMap<ParticleId, (PageId, PodId)>,
    /// Pod title-bar triggers: ParticleId → PodId (toggle collapse).
    pub pod_collapse_triggers: HashMap<ParticleId, PodId>,
    /// Per-page scroll viewport particle ids.
    pub page_viewport_ids: HashMap<PageId, ParticleId>,
    /// Page header split triggers: ParticleId → PageId.
    pub page_split_triggers: HashMap<ParticleId, PageId>,
    /// Page header editor-type switcher: ParticleId → PageId.
    pub page_template_menu_triggers: HashMap<ParticleId, PageId>,
    /// Hidden-page rail triggers: ParticleId → PageId (bring into focus).
    pub page_show_triggers: HashMap<ParticleId, PageId>,
    /// Size-class-scoped page size overrides from seam drags.
    pub page_overrides: Overrides,
    /// Focused page for cascade priority.
    pub focused_page: PageId,
    /// Analysis page header status source (live ratios).
    pub analysis_header_status_id: Option<ParticleId>,
    /// Graph UIView identity for this workspace container.
    pub node_id: NodeId,
}

impl StructuralWorkspace {
    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }

    pub fn focus_path(&self) -> FocusPath {
        FocusPath::new(vec![PageNode::container_id(self.focused_page)])
    }

    /// Resolve page cascade into `PageNode.state` and return Shown leaf rects.
    pub fn layout_pages(
        &mut self,
        pages_area: hyper_ui::Rect,
        app_focus: &FocusPath,
        viewport: &Viewport,
    ) -> (Vec<(PageId, hyper_ui::Rect)>, hyper_ui::ResolveReport) {
        // Sync focused_page from the app chain when it names a leaf in this tree.
        if let Some(page_id) = page_id_on_focus(app_focus, &self.page_tree) {
            self.focused_page = page_id;
        }
        let focus = self.focus_path();
        let (rects, report) =
            self.page_tree
                .layout(pages_area, &focus, &self.page_overrides, viewport);
        // If focus was hidden somehow, keep floor survivor focused.
        if let Some((id, _)) = rects.first() {
            if self
                .page_tree
                .find(self.focused_page)
                .map(|p| p.state.resolved() != hyper_ui::Visibility::Shown)
                .unwrap_or(true)
            {
                self.focused_page = *id;
            }
        }
        (rects, report)
    }

    /// Apply a seam drag between adjacent Shown pages at `seam_index`.
    pub fn apply_page_seam_drag(
        &mut self,
        seam_index: usize,
        ratio: f32,
        pages_area: hyper_ui::Rect,
    ) {
        let shown: Vec<_> = self
            .page_tree
            .leaves()
            .into_iter()
            .filter(|p| p.state.resolved() == hyper_ui::Visibility::Shown)
            .map(|p| p.id)
            .collect();
        if seam_index + 1 >= shown.len() {
            return;
        }
        let left = shown[seam_index];
        let right = shown[seam_index + 1];
        let class = LayoutSizeClass::from_width(pages_area.size.x.max(1.0));
        // `ratio` is the fraction of the two-page split_area for the left page.
        // Convert to fractions of the full arrangement axis using current widths.
        let rects = self.page_tree.leaf_rects(pages_area);
        let left_r = rects
            .iter()
            .find(|(id, _)| *id == left)
            .map(|(_, r)| r.size.x);
        let right_r = rects
            .iter()
            .find(|(id, _)| *id == right)
            .map(|(_, r)| r.size.x);
        let (Some(lw), Some(rw)) = (left_r, right_r) else {
            return;
        };
        let pair = (lw + rw).max(1.0);
        let new_left = (pair * ratio.clamp(0.1, 0.9)).max(1.0);
        let new_right = (pair - new_left).max(1.0);
        let denom = pages_area.size.x.max(1.0);
        self.page_overrides
            .set(PageNode::container_id(left), class, new_left / denom);
        self.page_overrides
            .set(PageNode::container_id(right), class, new_right / denom);
    }

    pub fn reset_page_seam(&mut self, seam_index: usize, pages_area: hyper_ui::Rect) {
        let shown: Vec<_> = self
            .page_tree
            .leaves()
            .into_iter()
            .filter(|p| p.state.resolved() == hyper_ui::Visibility::Shown)
            .map(|p| p.id)
            .collect();
        if seam_index + 1 >= shown.len() {
            return;
        }
        let class = LayoutSizeClass::from_width(pages_area.size.x.max(1.0));
        self.page_overrides
            .remove(PageNode::container_id(shown[seam_index]), class);
        self.page_overrides
            .remove(PageNode::container_id(shown[seam_index + 1]), class);
    }

    /// Resolve a pod rect by its template identifier across the page tree.
    pub fn pod_rect(
        &self,
        pages_area: hyper_ui::Rect,
        template: TemplateId,
    ) -> Option<hyper_ui::Rect> {
        for (page_id, page_rect) in self.page_tree.leaf_rects(pages_area) {
            let page = self.page_tree.find(page_id)?;
            let content = page.content_rect(page_rect);
            let leaves = page.pods.layout_rects(content);
            for ((template_page, pod_id), pod_template) in &self.pod_templates {
                if *template_page == page_id && *pod_template == template {
                    if let Some((_, r)) = leaves.iter().find(|(id, _)| *id == *pod_id) {
                        return Some(*r);
                    }
                }
            }
        }
        None
    }
}

fn page_id_on_focus(focus: &FocusPath, tree: &PageTree) -> Option<PageId> {
    for id in &focus.chain {
        for leaf in tree.leaves() {
            if PageNode::container_id(leaf.id) == *id {
                return Some(leaf.id);
            }
        }
    }
    None
}
