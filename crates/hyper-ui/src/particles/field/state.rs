#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldState {
    Idle,
    Editing,
    Invalid,
}
