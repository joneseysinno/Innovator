use crate::container::{ContainerState, Visibility};

use super::budget;
use super::ResolveReport;

/// Step 6 — squeeze Shown children when the floor still overflows.
pub(super) fn underflow(
    children: &[ContainerState],
    sizes: &mut [f32],
    effective_mins: &[f32],
    axis_available: f32,
    report: &mut ResolveReport,
) {
    budget::apply_underflow_sizes(children, sizes, effective_mins, axis_available);
    report.underflowed = true;
}

/// Steps 7a + 7b — raise toward ideal by want, then uncapped surplus by weight.
pub(super) fn surplus(
    children: &[ContainerState],
    sizes: &mut [f32],
    effective_ideals: &[f32],
    mut surplus: f32,
) {
    // 7a: fill toward ideal, proportional to remaining want.
    if surplus > f32::EPSILON {
        let mut total_want = 0.0;
        for (i, child) in children.iter().enumerate() {
            if child.resolved() != Visibility::Shown {
                continue;
            }
            total_want += (effective_ideals[i] - sizes[i]).max(0.0);
        }
        if total_want > f32::EPSILON {
            let raise = surplus.min(total_want);
            for (i, child) in children.iter().enumerate() {
                if child.resolved() != Visibility::Shown {
                    continue;
                }
                let want = (effective_ideals[i] - sizes[i]).max(0.0);
                let share = raise * (want / total_want);
                sizes[i] += share;
            }
            surplus -= raise;
        }
    }

    // 7b: remaining surplus by weight, uncapped.
    if surplus > f32::EPSILON {
        let total_weight: f32 = children
            .iter()
            .filter(|c| c.resolved() == Visibility::Shown)
            .map(|c| c.extent.weight.max(0.0))
            .sum();
        if total_weight > f32::EPSILON {
            for (i, child) in children.iter().enumerate() {
                if child.resolved() != Visibility::Shown {
                    continue;
                }
                let w = child.extent.weight.max(0.0);
                sizes[i] += surplus * (w / total_weight);
            }
        }
    }
}
