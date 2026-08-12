use crate::container::{FocusPath, Visibility};
use crate::geom::{Rect, Vec2};
use crate::layout::{InputClass, Overrides, SizeClass, Viewport, PAGE_LADDER};
use crate::page::{PageId, PageNode, PageTree};
use crate::{Extent, Pod, PodId, PodList};

fn three_page_tree() -> PageTree {
    let nav = PageNode::new(PageId(0), PodList::default())
        .with_label("Nav", "N")
        .with_extent(Extent::new(280.0, 360.0, 0.0));
    let analysis = PageNode::new(PageId(1), PodList::default())
        .with_label("Analysis", "A")
        .with_extent(Extent::new(400.0, 800.0, 1.0));
    let results = PageNode::new(PageId(2), PodList::default())
        .with_label("Results", "R")
        .with_extent(Extent::new(320.0, 560.0, 1.0));
    PageTree {
        pages: vec![nav, analysis, results],
    }
}

fn layout_at(tree: &mut PageTree, width: f32, focus_page: PageId, overrides: &Overrides) {
    let area = Rect::from_xywh(0.0, 0.0, width, 900.0);
    let viewport = Viewport {
        size: Vec2::new(width, 900.0),
        scale_factor: 1.0,
        size_class: SizeClass::from_width(width),
        input_class: InputClass::Pointer,
    };
    let focus = FocusPath::new(vec![PageNode::container_id(focus_page)]);
    let _ = tree.layout(area, &focus, overrides, &viewport);
}

fn shown_ids(tree: &PageTree) -> Vec<u64> {
    tree.leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == Visibility::Shown)
        .map(|p| p.id.0 as u64)
        .collect()
}

#[test]
fn at_1440_all_three_shown() {
    let mut tree = three_page_tree();
    layout_at(&mut tree, 1440.0, PageId(0), &Overrides::new());
    assert_eq!(shown_ids(&tree), vec![0, 1, 2]);
}

#[test]
fn at_834_furthest_from_focus_hides() {
    let mut tree = three_page_tree();
    layout_at(&mut tree, 834.0, PageId(0), &Overrides::new());
    assert_eq!(shown_ids(&tree), vec![0, 1]);
    assert_eq!(
        tree.find(PageId(2)).unwrap().state.resolved(),
        Visibility::Hidden
    );
}

#[test]
fn at_390_only_focused_page() {
    let mut tree = three_page_tree();
    layout_at(&mut tree, 390.0, PageId(0), &Overrides::new());
    assert_eq!(shown_ids(&tree), vec![0]);

    layout_at(&mut tree, 390.0, PageId(2), &Overrides::new());
    assert_eq!(shown_ids(&tree), vec![2]);
}

#[test]
fn seam_override_at_large_does_not_affect_compact() {
    let mut tree = three_page_tree();
    let mut overrides = Overrides::new();
    // Record a Large override that would starve siblings if applied at Compact.
    overrides.set(PageNode::container_id(PageId(0)), SizeClass::Large, 0.7);
    overrides.set(PageNode::container_id(PageId(1)), SizeClass::Large, 0.2);

    layout_at(&mut tree, 390.0, PageId(0), &overrides);
    // Compact resolve must ignore Large overrides — still floor to focused page.
    assert_eq!(shown_ids(&tree), vec![0]);
    let _ = PAGE_LADDER;
}

#[test]
fn shown_page_keeps_pods_scrollable() {
    let mut pods = PodList::new(
        (0..4)
            .map(|i| Pod::new(PodId(i), format!("P{i}")).with_height(1.0))
            .collect(),
    );
    let area = Rect::from_xywh(0.0, 0.0, 1440.0, 390.0);
    let (rects, report) = pods.layout(area);
    assert_eq!(rects.len(), 4);
    assert!(report.demotions.is_empty());
    assert!(report.scroll_extent > 0.0);
    assert!(pods.pods.iter().all(|p| p.state.resolved() == Visibility::Shown));
}
