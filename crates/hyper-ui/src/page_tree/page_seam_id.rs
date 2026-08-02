/// Identifies a Split node in a [`PageTree`] (pre-order seam index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageSeamId(pub u32);
