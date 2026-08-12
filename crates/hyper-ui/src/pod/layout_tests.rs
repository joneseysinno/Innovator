use crate::container::{ContainerId, FocusPath, Visibility};
use crate::geom::{Rect, Vec2};
use crate::layout::{InputClass, Overflow, SizeClass, Viewport, POD_LADDER};
use crate::pod::{Pod, PodId, PodList, default_collapse, resolved_collapse};

#[test]
fn two_pods_fill_tall_content_area() {
    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(0.30),
        Pod::new(PodId(1), "B").with_height(0.70),
    );
    let area = Rect::from_xywh(0.0, 0.0, 1440.0, 800.0);
    let (rects, report) = pods.layout(area);

    assert_eq!(report.scroll_extent, 0.0);
    let total: f32 = rects.iter().map(|(_, r)| r.size.y).sum::<f32>() + pods.gap;
    assert!(
        (total - area.size.y).abs() < 0.05,
        "pods+gap={total} area={}",
        area.size.y
    );
}

#[test]
fn eight_pods_at_compact_default_collapsed() {
    let mut pods = PodList::new(
        (0..8)
            .map(|i| Pod::new(PodId(i), format!("Pod {i}")).with_height(1.0))
            .collect(),
    );
    let area = Rect::from_xywh(0.0, 0.0, 300.0, 390.0);
    let (rects, report) = pods.layout(area);

    assert_eq!(rects.len(), 8);
    assert!(report.demotions.is_empty());
    for pod in &pods.pods {
        assert_eq!(pod.state.resolved(), Visibility::Collapsed);
        assert!(pod.collapsed);
    }
}

#[test]
fn collapse_writes_intent_not_system_demotion() {
    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(1.0),
        Pod::new(PodId(1), "B").with_height(1.0),
    );
    pods.toggle(PodId(1), SizeClass::Large);
    assert!(pods.pods[1].collapsed);
    assert_eq!(pods.pods[1].state.intent, Visibility::Collapsed);
    assert_eq!(
        pods.overrides
            .get_collapse(Pod::container_id(PodId(1)), SizeClass::Large),
        Some(true)
    );

    let area = Rect::from_xywh(0.0, 0.0, 1440.0, 200.0);
    let (_rects, report) = pods.layout(area);
    assert!(report.demotions.is_empty());
    assert_eq!(pods.pods[1].state.resolved(), Visibility::Collapsed);
    assert_eq!(pods.pods[0].state.resolved(), Visibility::Shown);
}

#[test]
fn anchor_scroll_keeps_title_stable() {
    let mut pods = PodList::new(
        (0..6)
            .map(|i| Pod::new(PodId(i), format!("P{i}")).with_height(1.0))
            .collect(),
    );
    let area = Rect::from_xywh(0.0, 0.0, 1440.0, 390.0);
    let (rects, _) = pods.layout(area);
    // Scroll so pod 4's title is near the top.
    let content_y = PodList::content_y_of(&rects, PodId(4), area).unwrap();
    pods.scroll_offset = content_y;
    let screen_y = content_y - pods.scroll_offset; // ~0

    pods.toggle(PodId(4), SizeClass::Large);
    pods.anchor_scroll_on_toggle(PodId(4), area, screen_y);

    let (after, _) = pods.layout(area);
    let new_y = PodList::content_y_of(&after, PodId(4), area).unwrap();
    let new_screen = new_y - pods.scroll_offset;
    assert!(
        (new_screen - screen_y).abs() < 1.0,
        "screen jumped: before={screen_y} after={new_screen}"
    );
}

#[test]
fn divider_drag_writes_override() {
    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(0.5),
        Pod::new(PodId(1), "B").with_height(0.5),
    );
    let class = SizeClass::Large;
    pods.apply_divider_drag(PodId(0), 40.0, 600.0, class);
    assert!(pods
        .overrides
        .get(Pod::container_id(PodId(0)), class)
        .is_some());
}

#[test]
fn large_width_keeps_pods_expanded_without_override() {
    let mut pods = PodList::new(
        (0..8)
            .map(|i| Pod::new(PodId(i), format!("Pod {i}")).with_height(1.0))
            .collect(),
    );
    for width in [2560.0, 1440.0, 1024.0] {
        let area = Rect::from_xywh(0.0, 0.0, width, 390.0);
        let vp = Viewport {
            size: Vec2::new(width, 390.0),
            scale_factor: 1.0,
            size_class: SizeClass::from_width(width),
            input_class: InputClass::Pointer,
        };
        let (_rects, report) = pods.layout_with(area, &vp, &FocusPath::default());
        assert!(report.demotions.is_empty(), "demoted at {width}");
        assert!(pods.pods.iter().all(|p| p.state.resolved() == Visibility::Shown));
        assert!(pods.pods.iter().all(|p| !p.collapsed));
    }
    let _ = (Overflow::Scroll, POD_LADDER, ContainerId(0));
}

#[test]
fn compact_defaults_collapsed_without_override() {
    assert!(default_collapse(PodId(0), SizeClass::Compact));
    assert!(!default_collapse(PodId(0), SizeClass::Medium));
    assert!(!default_collapse(PodId(0), SizeClass::Large));

    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(1.0),
        Pod::new(PodId(1), "B").with_height(1.0),
    );
    let area = Rect::from_xywh(0.0, 0.0, 390.0, 600.0);
    pods.layout(area);
    assert!(pods.pods.iter().all(|p| p.collapsed));
    assert!(pods
        .pods
        .iter()
        .all(|p| p.state.resolved() == Visibility::Collapsed));
}

#[test]
fn override_wins_over_compact_default() {
    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(1.0),
        Pod::new(PodId(1), "B").with_height(1.0),
    );
    pods.overrides
        .set_collapse(Pod::container_id(PodId(0)), SizeClass::Compact, false);
    let area = Rect::from_xywh(0.0, 0.0, 390.0, 600.0);
    pods.layout(area);
    assert!(!pods.pods[0].collapsed);
    assert!(pods.pods[1].collapsed);
    assert_eq!(
        resolved_collapse(PodId(0), SizeClass::Compact, &pods.overrides),
        false
    );
    assert_eq!(
        resolved_collapse(PodId(1), SizeClass::Compact, &pods.overrides),
        true
    );
}

#[test]
fn toggle_override_scoped_to_size_class() {
    let mut pods = PodList::two(
        Pod::new(PodId(0), "A").with_height(1.0),
        Pod::new(PodId(1), "B").with_height(1.0),
    );
    pods.toggle(PodId(1), SizeClass::Medium);
    assert_eq!(
        pods.overrides
            .get_collapse(Pod::container_id(PodId(1)), SizeClass::Medium),
        Some(true)
    );
    assert!(
        pods.overrides
            .get_collapse(Pod::container_id(PodId(1)), SizeClass::Large)
            .is_none()
    );

    let large = Rect::from_xywh(0.0, 0.0, 1440.0, 600.0);
    pods.layout(large);
    assert!(!pods.pods[1].collapsed, "Large default expands without override");

    let medium = Rect::from_xywh(0.0, 0.0, 800.0, 600.0);
    pods.layout(medium);
    assert!(pods.pods[1].collapsed, "Medium override sticks after toggle");
}
