use hyper_ui::{PageId, PageSeamId, PageSide, PodId, SeamDirection, TemplateId};

/// Signals that mutate the PageTree. Handled by the workspace, not the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSignal {
    Split {
        /// Split node whose child is being split, or parent of a leaf.
        seam_id: PageSeamId,
        direction: SeamDirection,
        /// Which side of the seam to split.
        side: PageSide,
    },
    /// Split a specific page leaf (header split trigger).
    SplitPage {
        page_id: PageId,
        direction: SeamDirection,
    },
    Merge {
        seam_id: PageSeamId,
        keep: PageSide,
    },
    ResetRatio {
        seam_id: PageSeamId,
    },
    ScrollToPod {
        page_id: PageId,
        pod_id: PodId,
    },
    /// Change a page's editor type (Blender-style area switch).
    SwitchTemplate {
        page_id: PageId,
        template_id: TemplateId,
    },
}
