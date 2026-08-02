pub mod analysis_page;
pub mod navigation_page;
pub mod results_page;

use crate::workspace::analysis::io_kind::IoKind;
use hyper_ui::{PageId, PageNode, PageTree, PodId, SeamDirection};

/// Initial three-page workspace layout matching the prior monolithic pod tree ratios.
pub fn initial_page_tree() -> (PageTree, std::collections::HashMap<PageId, Vec<(PodId, IoKind)>>) {
    let (nav, nav_ios) = navigation_page::navigation_page(PageId(0));
    let (analysis, analysis_ios) = analysis_page::analysis_page(PageId(1));
    let (results, results_ios) = results_page::results_page(PageId(2));

    let first = 0.22_f32.clamp(0.1, 0.8);
    let second = 0.48_f32.clamp(0.1, 0.8);
    let rest = (1.0 - first).max(0.2);
    let second_of_rest = (second / rest).clamp(0.1, 0.9);

    let tree = PageTree::Split {
        direction: SeamDirection::Vertical,
        ratio: first,
        first: Box::new(PageTree::Leaf(nav)),
        second: Box::new(PageTree::Split {
            direction: SeamDirection::Vertical,
            ratio: second_of_rest,
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

pub type PageTemplate = (PageNode, Vec<(PodId, IoKind)>);
