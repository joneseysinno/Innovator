use crate::geom::Vec2;
use crate::page::PageSeamId;
use crate::particles::field::FieldValue;
use crate::particles::{ParticleId, PointerKind};
use crate::pod::PodId;
use crate::seam::SeamDirection;

#[derive(Debug, Clone)]
pub enum UiEvent {
    TriggerFired(ParticleId),
    FieldCommit { id: ParticleId, value: FieldValue },
    FieldEditing { id: ParticleId, raw: String },
    SinkPointer {
        id: ParticleId,
        pos: Vec2,
        kind: PointerKind,
    },
    FocusChanged {
        from: Option<ParticleId>,
        to: Option<ParticleId>,
    },
    SeamDrag {
        seam_index: usize,
        delta: f32,
    },
    SeamReset {
        seam_index: usize,
    },
    /// Right-click on a page-level seam — application shows split/merge menu.
    PageSeamRightClick {
        seam_id: PageSeamId,
        cursor: Vec2,
        direction: SeamDirection,
    },
    /// Toggle collapse on a pod (title-bar click).
    PodCollapse {
        id: PodId,
    },
    /// Drag a pod divider — redistribute height between `above` and the next pod.
    PodDividerDrag {
        above: PodId,
        delta: f32,
    },
    /// Double-click a pod divider — equalize heights around `above`.
    PodDividerEqualize {
        above: PodId,
    },
}
