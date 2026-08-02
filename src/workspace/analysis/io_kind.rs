/// Application content kind assigned to a pod leaf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IoKind {
    WallList,
    WallSummary,
    InputForm,
    WallView,
    ResultsTable,
    Status,
    Empty,
}
