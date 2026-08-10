use crate::geom::Vec2;

/// Hysteresis margin (logical px) before a size-class boundary switch sticks.
pub const CLASS_SLOP: f32 = 32.0;

/// Width band lower bounds in logical pixels.
const MEDIUM_MIN: f32 = 640.0;
const EXPANDED_MIN: f32 = 1024.0;
const LARGE_MIN: f32 = 1440.0;

/// Logical viewport driving layout resolution.
///
/// `size` is always logical pixels (`physical / scale_factor`). Never physical.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub size: Vec2,
    pub scale_factor: f32,
    pub size_class: SizeClass,
    pub input_class: InputClass,
}

impl Viewport {
    pub fn new(
        size: Vec2,
        scale_factor: f32,
        previous_class: Option<SizeClass>,
        input_class: InputClass,
    ) -> Self {
        Self {
            size,
            scale_factor,
            size_class: SizeClass::from_width_hysteretic(size.x, previous_class),
            input_class,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SizeClass {
    Compact,
    Medium,
    Expanded,
    Large,
}

impl SizeClass {
    /// Classify `width` with no hysteresis.
    pub fn from_width(width: f32) -> Self {
        if width < MEDIUM_MIN {
            Self::Compact
        } else if width < EXPANDED_MIN {
            Self::Medium
        } else if width < LARGE_MIN {
            Self::Expanded
        } else {
            Self::Large
        }
    }

    /// Lower bound (inclusive) of this class's width band.
    pub fn lower_bound(self) -> f32 {
        match self {
            Self::Compact => 0.0,
            Self::Medium => MEDIUM_MIN,
            Self::Expanded => EXPANDED_MIN,
            Self::Large => LARGE_MIN,
        }
    }

    /// Classify `width`, requiring a `CLASS_SLOP` overshoot before leaving
    /// `previous`. Without this, a slow drag across a boundary thrashes layout.
    pub fn from_width_hysteretic(width: f32, previous: Option<SizeClass>) -> Self {
        let naive = Self::from_width(width);
        let Some(prev) = previous else {
            return naive;
        };
        if naive == prev {
            return prev;
        }
        if naive > prev {
            // Promote only after clearing the new class's lower bound by slop.
            if width >= naive.lower_bound() + CLASS_SLOP {
                naive
            } else {
                prev
            }
        } else {
            // Demote only after falling CLASS_SLOP below the previous lower bound.
            if width < prev.lower_bound() - CLASS_SLOP {
                naive
            } else {
                prev
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InputClass {
    #[default]
    Pointer,
    Touch,
    Hybrid,
}

impl InputClass {
    /// Absolute minimum interactive target size in logical pixels.
    pub fn min_target(self) -> f32 {
        match self {
            Self::Pointer => 0.0,
            Self::Touch | Self::Hybrid => 44.0,
        }
    }

    /// Extra hit padding around seams / thin chrome, in logical pixels.
    pub fn hit_slop(self) -> f32 {
        match self {
            Self::Pointer => 4.0,
            Self::Touch | Self::Hybrid => 12.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InputClass, SizeClass, CLASS_SLOP, EXPANDED_MIN, LARGE_MIN, MEDIUM_MIN};

    #[test]
    fn naive_bands() {
        assert_eq!(SizeClass::from_width(639.0), SizeClass::Compact);
        assert_eq!(SizeClass::from_width(640.0), SizeClass::Medium);
        assert_eq!(SizeClass::from_width(1023.0), SizeClass::Medium);
        assert_eq!(SizeClass::from_width(1024.0), SizeClass::Expanded);
        assert_eq!(SizeClass::from_width(1439.0), SizeClass::Expanded);
        assert_eq!(SizeClass::from_width(1440.0), SizeClass::Large);
    }

    #[test]
    fn up_sweep_600_to_1100_crosses_each_boundary_once() {
        let mut class = SizeClass::from_width_hysteretic(600.0, None);
        assert_eq!(class, SizeClass::Compact);

        let mut crossings = Vec::new();
        for w in (600..=1100).map(|n| n as f32) {
            let next = SizeClass::from_width_hysteretic(w, Some(class));
            if next != class {
                crossings.push((w, class, next));
                class = next;
            }
        }

        assert_eq!(
            crossings,
            vec![
                (MEDIUM_MIN + CLASS_SLOP, SizeClass::Compact, SizeClass::Medium),
                (EXPANDED_MIN + CLASS_SLOP, SizeClass::Medium, SizeClass::Expanded),
            ]
        );
        assert_eq!(class, SizeClass::Expanded);
    }

    #[test]
    fn down_sweep_1100_to_600_crosses_each_boundary_once() {
        let mut class = SizeClass::Expanded;
        let mut crossings = Vec::new();
        for w in (600..=1100).rev().map(|n| n as f32) {
            let next = SizeClass::from_width_hysteretic(w, Some(class));
            if next != class {
                crossings.push((w, class, next));
                class = next;
            }
        }

        // First width that is strictly below bound - CLASS_SLOP.
        let expanded_exit = EXPANDED_MIN - CLASS_SLOP - 1.0; // 991
        let medium_exit = MEDIUM_MIN - CLASS_SLOP - 1.0; // 607
        assert_eq!(
            crossings,
            vec![
                (expanded_exit, SizeClass::Expanded, SizeClass::Medium),
                (medium_exit, SizeClass::Medium, SizeClass::Compact),
            ]
        );
        assert_eq!(class, SizeClass::Compact);
    }

    #[test]
    fn full_sweep_600_1100_600_never_oscillates_in_slop() {
        let mut class = SizeClass::from_width_hysteretic(600.0, None);
        let mut history = vec![(600.0_f32, class)];

        for w in (600..=1100).chain((600..=1100).rev()).map(|n| n as f32) {
            let next = SizeClass::from_width_hysteretic(w, Some(class));
            if next != class {
                history.push((w, next));
                class = next;
            }
        }

        // Exactly four transitions: up Compact→Medium→Expanded, down Expanded→Medium→Compact.
        assert_eq!(history.len(), 5); // start + 4 crossings
        assert_eq!(history[1].1, SizeClass::Medium);
        assert_eq!(history[2].1, SizeClass::Expanded);
        assert_eq!(history[3].1, SizeClass::Medium);
        assert_eq!(history[4].1, SizeClass::Compact);

        // Oscillation inside CLASS_SLOP of the Expanded boundary must not flip.
        let mut stuck = SizeClass::Medium;
        for w in ((EXPANDED_MIN as i32 - CLASS_SLOP as i32 + 1)
            ..=(EXPANDED_MIN as i32 + CLASS_SLOP as i32 - 1))
            .map(|n| n as f32)
        {
            let next = SizeClass::from_width_hysteretic(w, Some(stuck));
            assert_eq!(next, SizeClass::Medium, "oscillated at width {w}");
            stuck = next;
        }
    }

    #[test]
    fn promote_to_large_requires_slop() {
        let at_bound = SizeClass::from_width_hysteretic(LARGE_MIN, Some(SizeClass::Expanded));
        assert_eq!(at_bound, SizeClass::Expanded);
        let past = SizeClass::from_width_hysteretic(LARGE_MIN + CLASS_SLOP, Some(SizeClass::Expanded));
        assert_eq!(past, SizeClass::Large);
    }

    #[test]
    fn input_class_targets() {
        assert_eq!(InputClass::Pointer.min_target(), 0.0);
        assert_eq!(InputClass::Touch.min_target(), 44.0);
        assert_eq!(InputClass::Hybrid.min_target(), 44.0);
        assert_eq!(InputClass::Pointer.hit_slop(), 4.0);
        assert_eq!(InputClass::Touch.hit_slop(), 12.0);
        assert_eq!(InputClass::Hybrid.hit_slop(), 12.0);
    }
}
