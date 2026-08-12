/// Opaque page/pod template key — replaces closed enum dispatch.
///
/// String values like `"analysis"` are domain concepts and must be defined
/// only in the app (`Innovator/src/domains/…`), never inside `hyper-ui`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateId(pub &'static str);

impl TemplateId {
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
