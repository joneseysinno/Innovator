use crate::container::{ContainerId, ContainerState, Extent, FocusPath, Visibility};
use crate::geom::Vec2;
use crate::layout::{
    resolve, Axis, InputClass, Overflow, Overrides, SizeClass, Viewport, PAGE_LADDER, POD_LADDER,
};

/// Three-page fixture: nav / analysis / results. Sum of mins = 1000.
fn three_pages() -> Vec<ContainerState> {
    vec![
        ContainerState::new(
            ContainerId(0),
            "Nav",
            "N",
            Visibility::Shown,
            Extent::new(280.0, 360.0, 0.0),
        ),
        ContainerState::new(
            ContainerId(1),
            "Analysis",
            "A",
            Visibility::Shown,
            Extent::new(400.0, 800.0, 1.0),
        ),
        ContainerState::new(
            ContainerId(2),
            "Results",
            "R",
            Visibility::Shown,
            Extent::new(320.0, 560.0, 1.0),
        ),
    ]
}

fn viewport(width: f32) -> Viewport {
    Viewport {
        size: Vec2::new(width, 900.0),
        scale_factor: 1.0,
        size_class: SizeClass::from_width(width),
        input_class: InputClass::Pointer,
    }
}

fn resolve_pages(
    pages: &mut [ContainerState],
    width: f32,
    focus_index: usize,
) -> crate::layout::ResolveReport {
    let focus = FocusPath::new(vec![pages[focus_index].id]);
    let vp = viewport(width);
    resolve(
        pages,
        width,
        900.0,
        Axis::Horizontal,
        Overflow::Cascade,
        PAGE_LADDER,
        1,
        &focus,
        &Overrides::new(),
        &vp,
    )
}

fn shown_ids(pages: &[ContainerState]) -> Vec<u64> {
    pages
        .iter()
        .filter(|p| p.resolved() == Visibility::Shown)
        .map(|p| p.id.0)
        .collect()
}

fn approx(a: f32, b: f32) {
    assert!(
        (a - b).abs() < 0.05,
        "expected {b}, got {a} (delta {})",
        (a - b).abs()
    );
}

struct Expectation {
    width: f32,
    survivors: &'static [u64],
    /// Axis sizes for survivors in survivor order; Hidden get 0.
    sizes: &'static [f32],
    underflowed: bool,
}

#[test]
fn table_driven_three_pages_focus_nav() {
    // Precomputed from min+ideal+weight algorithm (see plan Phase 3).
    let cases = [
        Expectation {
            width: 2560.0,
            survivors: &[0, 1, 2],
            sizes: &[360.0, 1220.0, 980.0],
            underflowed: false,
        },
        Expectation {
            width: 1440.0,
            survivors: &[0, 1, 2],
            sizes: &[328.889, 644.444, 466.667],
            underflowed: false,
        },
        Expectation {
            width: 1112.0,
            survivors: &[0, 1, 2],
            sizes: &[292.444, 462.222, 357.333],
            underflowed: false,
        },
        Expectation {
            width: 834.0,
            survivors: &[0, 1],
            sizes: &[305.667, 528.333, 0.0],
            underflowed: false,
        },
        Expectation {
            width: 640.0,
            survivors: &[0],
            // Weight-0 sole survivor still fills the axis (7c stretch).
            sizes: &[640.0, 0.0, 0.0],
            underflowed: false,
        },
        Expectation {
            width: 390.0,
            survivors: &[0],
            sizes: &[390.0, 0.0, 0.0],
            underflowed: false,
        },
        Expectation {
            width: 200.0,
            survivors: &[0],
            sizes: &[200.0, 0.0, 0.0],
            underflowed: true,
        },
    ];

    for case in cases {
        let mut pages = three_pages();
        let report = resolve_pages(&mut pages, case.width, 0);
        assert_eq!(
            shown_ids(&pages),
            case.survivors,
            "survivors at width {}",
            case.width
        );
        assert_eq!(report.underflowed, case.underflowed, "underflow at {}", case.width);
        assert!(
            shown_ids(&pages).len() >= 1,
            "floor breached at {}",
            case.width
        );
        for (i, &expected) in case.sizes.iter().enumerate() {
            approx(pages[i].rect().size.x, expected);
        }
        let sum: f32 = pages.iter().map(|p| p.rect().size.x).sum();
        // Shown pages always consume the full axis (Cascade fill).
        approx(sum, case.width);
    }
}

#[test]
fn focus_index_changes_survivors_at_390() {
    let mut focus_nav = three_pages();
    resolve_pages(&mut focus_nav, 390.0, 0);
    assert_eq!(shown_ids(&focus_nav), vec![0]);

    let mut focus_results = three_pages();
    resolve_pages(&mut focus_results, 390.0, 2);
    assert_eq!(shown_ids(&focus_results), vec![2]);
    approx(focus_results[2].rect().size.x, 390.0);
}

#[test]
fn floor_never_breached_across_widths() {
    for width in [2560.0, 1440.0, 1112.0, 834.0, 640.0, 390.0, 200.0, 100.0] {
        let mut pages = three_pages();
        resolve_pages(&mut pages, width, 0);
        let visible = pages
            .iter()
            .filter(|p| p.resolved() != Visibility::Hidden)
            .count();
        assert!(visible >= 1, "floor breached at {width}");
        assert!(!pages.iter().any(|p| p.resolved() == Visibility::Collapsed));
    }
}

#[test]
fn underflow_at_200_without_panic() {
    let mut pages = three_pages();
    let report = resolve_pages(&mut pages, 200.0, 0);
    assert!(report.underflowed);
    assert_eq!(shown_ids(&pages), vec![0]);
    approx(pages[0].rect().size.x, 200.0);
    // Squeezed below min but not below min * UNDERFLOW_FACTOR (196).
    assert!(pages[0].rect().size.x >= 280.0 * 0.7 - 0.05);
}

#[test]
fn scroll_surplus_fills_axis_when_content_fits() {
    // Two pods: ideals 144 + 336 = 480, weights 0.3 / 0.7. Tall budget → fill.
    let mut pods = vec![
        ContainerState::new(
            ContainerId(100),
            "Input",
            "P",
            Visibility::Shown,
            Extent::new(80.0, 144.0, 0.3),
        ),
        ContainerState::new(
            ContainerId(101),
            "Wall",
            "P",
            Visibility::Shown,
            Extent::new(80.0, 336.0, 0.7),
        ),
    ];
    let focus = FocusPath::new(vec![ContainerId(100)]);
    let budget = 800.0;
    let vp = viewport(budget);
    let report = resolve(
        &mut pods,
        budget,
        400.0,
        Axis::Vertical,
        Overflow::Scroll,
        POD_LADDER,
        0,
        &focus,
        &Overrides::new(),
        &vp,
    );

    assert_eq!(report.scroll_extent, 0.0);
    assert!(!report.underflowed);
    let h0 = pods[0].rect().size.y;
    let h1 = pods[1].rect().size.y;
    approx(h0 + h1, budget);
    // Surplus past ideals (320) split 0.3/0.7 → +96 / +224.
    approx(h0, 144.0 + 96.0);
    approx(h1, 336.0 + 224.0);
}

#[test]
fn eight_pods_scroll_at_390_all_shown() {
    let mut pods: Vec<ContainerState> = (0..8)
        .map(|i| {
            ContainerState::new(
                ContainerId(100 + i),
                format!("Pod {i}"),
                "P",
                Visibility::Shown,
                Extent::new(80.0, 100.0, 1.0),
            )
        })
        .collect();

    let focus = FocusPath::new(vec![ContainerId(100)]);
    let vp = viewport(390.0);
    let report = resolve(
        &mut pods,
        390.0,
        390.0,
        Axis::Vertical,
        Overflow::Scroll,
        POD_LADDER,
        0,
        &focus,
        &Overrides::new(),
        &vp,
    );

    assert!(report.demotions.is_empty(), "scroll must not demote");
    assert_eq!(report.scroll_extent, 800.0 - 390.0);
    assert!(!report.underflowed);
    for pod in &pods {
        assert_eq!(pod.resolved(), Visibility::Shown);
        approx(pod.rect().size.y, 100.0);
    }
}

#[test]
fn override_bumps_ideal_within_size_class() {
    let mut pages = three_pages();
    let mut overrides = Overrides::new();
    // At Large, give analysis half the axis.
    overrides.set(ContainerId(1), SizeClass::Large, 0.5);

    let focus = FocusPath::new(vec![ContainerId(0)]);
    let vp = viewport(2560.0);
    assert_eq!(vp.size_class, SizeClass::Large);

    resolve(
        &mut pages,
        2560.0,
        900.0,
        Axis::Horizontal,
        Overflow::Cascade,
        PAGE_LADDER,
        1,
        &focus,
        &overrides,
        &vp,
    );

    // Analysis effective ideal = max(400, 0.5*2560) = 1280.
    // After fill-to-ideal + weight split, analysis should be >= 1280.
    assert!(pages[1].rect().size.x >= 1279.0);
}

#[test]
fn promote_requires_slop_after_demotion() {
    use crate::layout::PROMOTE_SLOP;

    let mut pages = three_pages();
    resolve_pages(&mut pages, 390.0, 0);
    assert_eq!(shown_ids(&pages), vec![0]);

    // Nav + Analysis mins = 680. Exactly 680 fits two pages when starting
    // fresh, but after demotion sticky + PROMOTE_SLOP, promotion needs
    // 680 + 24.
    resolve_pages(&mut pages, 680.0, 0);
    assert_eq!(
        shown_ids(&pages),
        vec![0],
        "must stay demoted without PROMOTE_SLOP headroom"
    );

    resolve_pages(&mut pages, 680.0 + PROMOTE_SLOP, 0);
    assert_eq!(shown_ids(&pages), vec![0, 1]);
}

#[test]
fn fresh_resolve_at_680_shows_two_without_prior_demotion() {
    let mut pages = three_pages();
    resolve_pages(&mut pages, 680.0, 0);
    assert_eq!(shown_ids(&pages), vec![0, 1]);
}
