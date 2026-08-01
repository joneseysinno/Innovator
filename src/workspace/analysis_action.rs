/// Analysis-page local actions (InputForm / FieldBuilder).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisAction {
    OpenFieldBuilder,
    CancelFieldBuilder,
    ConfirmFieldBuilder,
    FieldKindNumber,
    FieldKindText,
    FieldKindBool,
}
