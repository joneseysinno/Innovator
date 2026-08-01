use crate::geom::Vec2;
use crate::particles::field::FieldValue;
use crate::particles::{ParticleId, PointerKind};

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
}
