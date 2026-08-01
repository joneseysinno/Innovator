#[derive(Clone, Copy)]
pub(crate) enum HitKind {
    Trigger,
    Field,
    Sink,
    Other,
}
