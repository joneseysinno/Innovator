use crate::container::{ContainerId, ContainerState, FocusPath, Visibility};

use super::super::DemotionLadder;
use super::{budget, ResolveReport};

/// Extra axis headroom (logical px) required before a cascade-demoted child
/// may promote back. Prevents flicker when dragging across a fit boundary.
pub const PROMOTE_SLOP: f32 = 24.0;

/// Cascade demotion + hysteretic promotion (algorithm steps 3 + 5).
pub(super) fn run(
    children: &mut [ContainerState],
    effective_mins: &[f32],
    axis_available: f32,
    ladder: DemotionLadder,
    floor: usize,
    focus: &FocusPath,
    report: &mut ResolveReport,
) {
    let ids: Vec<ContainerId> = children.iter().map(|c| c.id).collect();

    // Demote while over budget.
    loop {
        let budget = budget::compute(children, effective_mins, ladder.collapsed_extent);
        if budget <= axis_available {
            break;
        }

        let Some(victim) = pick_victim(children, &ids, focus, ladder, floor) else {
            break;
        };

        let from = children[victim].resolved();
        let to = ladder.demote(from);
        if to == from {
            break;
        }
        children[victim].set_resolved(to);
        report.demotions.push((children[victim].id, from, to));
    }

    // Promote cascade-demoted children only with PROMOTE_SLOP headroom.
    loop {
        let budget = budget::compute(children, effective_mins, ladder.collapsed_extent);
        let Some(candidate) = pick_promote(children, &ids, focus, ladder) else {
            break;
        };

        let from = children[candidate].resolved();
        let to = ladder.promote(from);
        // Must become more visible (`to < from`) and not exceed user intent
        // (`to >= intent` under Ord where Shown is least demoted).
        if to >= from || to < children[candidate].intent {
            break;
        }

        let cost = visibility_extent(to, effective_mins[candidate], ladder.collapsed_extent)
            - visibility_extent(from, effective_mins[candidate], ladder.collapsed_extent);
        if budget + cost + PROMOTE_SLOP > axis_available {
            break;
        }

        children[candidate].set_resolved(to);
        report.promotions.push((children[candidate].id, from, to));
    }
}

fn visibility_extent(v: Visibility, min: f32, collapsed: f32) -> f32 {
    match v {
        Visibility::Shown => min,
        Visibility::Collapsed => collapsed,
        Visibility::Hidden => 0.0,
    }
}

/// Shown child with greatest focus distance; ties → higher index.
/// Skips focus-path members and demotions that would breach `floor`.
fn pick_victim(
    children: &[ContainerState],
    ids: &[ContainerId],
    focus: &FocusPath,
    ladder: DemotionLadder,
    floor: usize,
) -> Option<usize> {
    let visible = budget::visible_count(children);
    let mut best: Option<(usize, usize)> = None; // (distance, index)

    for (i, child) in children.iter().enumerate() {
        if child.resolved() != Visibility::Shown {
            continue;
        }
        if focus.contains(child.id) {
            continue;
        }

        let next = ladder.demote(child.resolved());
        if next == child.resolved() {
            continue;
        }
        // Floor: children must remain at least Collapsed (not Hidden).
        if next == Visibility::Hidden && visible <= floor {
            continue;
        }

        let dist = focus.distance(i, ids);
        match best {
            None => best = Some((dist, i)),
            Some((best_dist, best_i)) => {
                if dist > best_dist || (dist == best_dist && i > best_i) {
                    best = Some((dist, i));
                }
            }
        }
    }

    best.map(|(_, i)| i)
}

/// Demoted child (resolved > intent toward Hidden) closest to focus; ties →
/// lower index. Only candidates whose intent is more visible than resolved.
fn pick_promote(
    children: &[ContainerState],
    ids: &[ContainerId],
    focus: &FocusPath,
    ladder: DemotionLadder,
) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None; // (distance, index)

    for (i, child) in children.iter().enumerate() {
        // Demoted below intent (e.g. intent Shown, resolved Hidden).
        if child.resolved() <= child.intent {
            continue;
        }
        let next = ladder.promote(child.resolved());
        if next == child.resolved() || next > child.intent {
            continue;
        }

        let dist = focus.distance(i, ids);
        match best {
            None => best = Some((dist, i)),
            Some((best_dist, best_i)) => {
                if dist < best_dist || (dist == best_dist && i < best_i) {
                    best = Some((dist, i));
                }
            }
        }
    }

    best.map(|(_, i)| i)
}
