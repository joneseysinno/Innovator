use crate::container::{ContainerState, Visibility};

use super::UNDERFLOW_FACTOR;

/// Min-budget for the current resolved visibilities.
pub(super) fn compute(
    children: &[ContainerState],
    effective_mins: &[f32],
    collapsed_extent: f32,
) -> f32 {
    children
        .iter()
        .enumerate()
        .map(|(i, child)| match child.resolved() {
            Visibility::Shown => effective_mins[i],
            Visibility::Collapsed => collapsed_extent,
            Visibility::Hidden => 0.0,
        })
        .sum()
}

/// Count of children still at least Collapsed (i.e. not Hidden).
pub(super) fn visible_count(children: &[ContainerState]) -> usize {
    children
        .iter()
        .filter(|c| c.resolved() != Visibility::Hidden)
        .count()
}

/// Shrink Shown allocations toward `min * UNDERFLOW_FACTOR` until the sum
/// fits `axis_available`, or every Shown child is at its underflow floor.
pub(super) fn apply_underflow_sizes(
    children: &[ContainerState],
    sizes: &mut [f32],
    effective_mins: &[f32],
    axis_available: f32,
) {
    let mut guard = 0;
    while sizes.iter().sum::<f32>() > axis_available + f32::EPSILON && guard < 64 {
        guard += 1;
        let excess = sizes.iter().sum::<f32>() - axis_available;
        let mut shrinkable = 0.0;
        for (i, child) in children.iter().enumerate() {
            if child.resolved() != Visibility::Shown {
                continue;
            }
            let floor = effective_mins[i] * UNDERFLOW_FACTOR;
            shrinkable += (sizes[i] - floor).max(0.0);
        }
        if shrinkable <= f32::EPSILON {
            break;
        }
        for (i, child) in children.iter().enumerate() {
            if child.resolved() != Visibility::Shown {
                continue;
            }
            let floor = effective_mins[i] * UNDERFLOW_FACTOR;
            let room = (sizes[i] - floor).max(0.0);
            let share = excess * (room / shrinkable);
            sizes[i] = (sizes[i] - share).max(floor);
        }
    }
}
