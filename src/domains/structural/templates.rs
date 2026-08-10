pub mod analysis_page;
pub mod navigation_page;
pub mod results_page;

use super::io_kind::IoKind;
use hyper_ui::{Extent, PageId, PageTree, PodId, SeamDirection};

/// Initial three-page workspace layout with authored extents for cascade resolve.
pub fn initial_page_tree() -> (PageTree, std::collections::HashMap<PageId, Vec<(PodId, IoKind)>>) {
    let (nav, nav_ios) = navigation_page::navigation_page(PageId(0));
    let (analysis, analysis_ios) = analysis_page::analysis_page(PageId(1));
    let (results, results_ios) = results_page::results_page(PageId(2));

    // Topology only — sizes come from ContainerState extents + Overrides.
    let tree = PageTree::Split {
        direction: SeamDirection::Vertical,
        first: Box::new(PageTree::Leaf(nav)),
        second: Box::new(PageTree::Split {
            direction: SeamDirection::Vertical,
            first: Box::new(PageTree::Leaf(analysis)),
            second: Box::new(PageTree::Leaf(results)),
        }),
    };

    let mut page_ios = std::collections::HashMap::new();
    page_ios.insert(PageId(0), nav_ios);
    page_ios.insert(PageId(1), analysis_ios);
    page_ios.insert(PageId(2), results_ios);

    (tree, page_ios)
}

/// Empty page assignment used after a split.
pub fn empty_page_ios() -> Vec<(PodId, IoKind)> {
    vec![(PodId(0), IoKind::Empty)]
}

pub type PageTemplate = (hyper_ui::PageNode, Vec<(PodId, IoKind)>);

/// Extent fixtures matching Phase 3 resolve acceptance tests.
pub fn nav_extent() -> Extent {
    Extent::new(280.0, 360.0, 0.0)
}
pub fn analysis_extent() -> Extent {
    Extent::new(400.0, 800.0, 1.0)
}
pub fn results_extent() -> Extent {
    Extent::new(320.0, 560.0, 1.0)
}
