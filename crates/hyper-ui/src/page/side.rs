/// Which child of a page-tree Split to keep on merge (or target on split).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageSide {
    First,
    Second,
}
