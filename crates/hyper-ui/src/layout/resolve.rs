//! Pure resolve pass — demands → visibility + rects.

mod assign_rects;
mod budget;
mod cascade;
mod distribute;

use crate::container::{ContainerState, FocusPath, Visibility};
use crate::geom::Vec2;

use super::{
    DemotionLadder, Overflow, Overrides, Viewport,
};

pub use assign_rects::Axis;
pub use cascade::PROMOTE_SLOP;

/// Shrink floor under cascade underflow: `min * UNDERFLOW_FACTOR`.
pub const UNDERFLOW_FACTOR: f32 = 0.7;

/// Outcome of one [`resolve`] call — demotions, promotions, scroll, underflow.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolveReport {
    /// `(id, from, to)` for every cascade demotion this pass.
    pub demotions: Vec<(crate::container::ContainerId, Visibility, Visibility)>,
    /// `(id, from, to)` for every hysteretic promotion this pass.
    pub promotions: Vec<(crate::container::ContainerId, Visibility, Visibility)>,
    /// Content extent beyond `axis_available` when [`Overflow::Scroll`].
    pub scroll_extent: f32,
    /// True when the floor still could not fit and Shown children were squeezed.
    pub underflowed: bool,
}

impl ResolveReport {
    fn empty() -> Self {
        Self {
            demotions: Vec::new(),
            promotions: Vec::new(),
            scroll_extent: 0.0,
            underflowed: false,
        }
    }
}

/// Resolve one arrangement's children against an available extent.
///
/// Pure: no globals, no GPU, no window. Fully unit-testable.
///
/// **Ideal settlement:** `ideal` is kept. It is the Scroll allocation size and
/// the soft target for surplus step 7a. Surplus beyond ideals (7b) uses
/// `weight` only. A layout that authors `ideal == min` behaves as min+weight.
pub fn resolve(
    children: &mut [ContainerState],
    axis_available: f32,
    cross_extent: f32,
    axis: Axis,
    overflow: Overflow,
    ladder: DemotionLadder,
    floor: usize,
    focus: &FocusPath,
    overrides: &Overrides,
    viewport: &Viewport,
) -> ResolveReport {
    let mut report = ResolveReport::empty();

    // 1. Seed resolved from intent. Under Cascade, keep a prior demotion sticky
    //    when intent is still Shown so PROMOTE_SLOP can gate restoration — except
    //    for containers on the focus path, which always get a fresh chance.
    match overflow {
        Overflow::Scroll => {
            for child in children.iter_mut() {
                child.set_resolved(child.intent);
            }
        }
        Overflow::Cascade => {
            for child in children.iter_mut() {
                let previous = child.resolved();
                if focus.contains(child.id) {
                    child.set_resolved(child.intent);
                } else if child.intent > Visibility::Shown {
                    // User collapsed/hid — honor intent.
                    child.set_resolved(child.intent);
                } else if previous > child.intent {
                    // Cascade-demoted last pass; leave for promote loop.
                } else {
                    child.set_resolved(child.intent);
                }
            }
        }
    }

    // Effective mins / ideals for this viewport + overrides.
    let effective_mins = effective_mins(children, viewport);
    let effective_ideals =
        effective_ideals(children, overrides, viewport.size_class, axis_available, &effective_mins);

    let mut sizes = vec![0.0_f32; children.len()];

    match overflow {
        Overflow::Scroll => {
            // 4. No demotion. Allocate effective ideals; scroll the excess.
            let mut total = 0.0;
            for (i, child) in children.iter().enumerate() {
                sizes[i] = match child.resolved() {
                    Visibility::Shown => effective_ideals[i],
                    Visibility::Collapsed => ladder.collapsed_extent,
                    Visibility::Hidden => 0.0,
                };
                total += sizes[i];
            }
            report.scroll_extent = (total - axis_available).max(0.0);
            // 7. When content fits, distribute leftover so the axis is filled.
            //    Under Scroll with positive scroll_extent there is no surplus.
            if report.scroll_extent <= 0.0 {
                let surplus = axis_available - total;
                if surplus > 0.0 {
                    distribute::surplus(children, &mut sizes, &effective_ideals, surplus);
                }
            }
        }
        Overflow::Cascade => {
            // 3 + 5. Budget and demote until it fits or floor reached.
            cascade::run(
                children,
                &effective_mins,
                axis_available,
                ladder,
                floor,
                focus,
                &mut report,
            );

            // Seed sizes from post-cascade visibility.
            for (i, child) in children.iter().enumerate() {
                sizes[i] = match child.resolved() {
                    Visibility::Shown => effective_mins[i],
                    Visibility::Collapsed => ladder.collapsed_extent,
                    Visibility::Hidden => 0.0,
                };
            }

            let budget: f32 = sizes.iter().sum();
            if budget > axis_available {
                // 6. Underflow squeeze.
                distribute::underflow(
                    children,
                    &mut sizes,
                    &effective_mins,
                    axis_available,
                    &mut report,
                );
            } else {
                // 7. Distribute surplus toward ideal, then by weight.
                let surplus = axis_available - budget;
                if surplus > 0.0 {
                    distribute::surplus(
                        children,
                        &mut sizes,
                        &effective_ideals,
                        surplus,
                    );
                }
            }
        }
    }

    // 8. Assign rects.
    assign_rects::assign(children, &sizes, cross_extent, axis, Vec2::ZERO);

    report
}

fn effective_mins(children: &[ContainerState], viewport: &Viewport) -> Vec<f32> {
    let floor = viewport.input_class.min_target();
    children
        .iter()
        .map(|c| c.extent.min.max(floor))
        .collect()
}

fn effective_ideals(
    children: &[ContainerState],
    overrides: &Overrides,
    class: super::SizeClass,
    axis_available: f32,
    mins: &[f32],
) -> Vec<f32> {
    children
        .iter()
        .enumerate()
        .map(|(i, c)| {
            overrides.effective_ideal(c.id, class, axis_available, mins[i], c.extent.ideal)
        })
        .collect()
}

#[cfg(test)]
mod tests;