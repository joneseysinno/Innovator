/// Kind of analysis hosted inside an Analysis workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnalysisKind {
    /// Special concrete wall (first analysis type).
    #[default]
    SpecialConcreteWall,
}
