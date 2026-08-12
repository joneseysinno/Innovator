//! Page-template interface used by graph-backed UIView pages.

use crate::domains::structural::StructuralWorkspace;
use hyper_ui::{particles::Particle, PageHeaderSlots, PageId, PageNode, TemplateId};
use hypernode::Graph;

/// Runtime data required by a page template builder.
pub struct TemplateCtx<'a> {
    pub workspace: &'a mut StructuralWorkspace,
    pub graph: &'a Graph,
    pub page: &'a PageNode,
    pub page_id: PageId,
}

pub trait PageTemplate: Send + Sync {
    fn id(&self) -> TemplateId;
    fn header_slots(&self) -> PageHeaderSlots;
    fn build_header(&self, ctx: &mut TemplateCtx<'_>) -> Particle;
    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle;
}
