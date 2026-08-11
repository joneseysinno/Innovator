//! F9 simulated viewport presets, letterboxed inside the real window.

use hyper_ui::{letterbox_rect, InputClass, Rect, Vec2};

/// Cycles with F9. `Native` uses the real window; others force size + input class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewPreset {
    #[default]
    Native,
    Phone,
    PhoneLandscape,
    Tablet,
    TabletLandscape,
    Laptop,
}

impl PreviewPreset {
    pub const ALL: [PreviewPreset; 6] = [
        PreviewPreset::Native,
        PreviewPreset::Phone,
        PreviewPreset::PhoneLandscape,
        PreviewPreset::Tablet,
        PreviewPreset::TabletLandscape,
        PreviewPreset::Laptop,
    ];

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|p| *p == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Native => "Native",
            Self::Phone => "Phone 390×844 Touch",
            Self::PhoneLandscape => "Phone landscape 844×390 Touch",
            Self::Tablet => "Tablet 834×1112 Touch",
            Self::TabletLandscape => "Tablet landscape 1112×834 Touch",
            Self::Laptop => "Laptop 1440×900 Pointer",
        }
    }

    /// Logical size for non-Native presets.
    pub fn size(self) -> Option<Vec2> {
        Some(match self {
            Self::Native => return None,
            Self::Phone => Vec2::new(390.0, 844.0),
            Self::PhoneLandscape => Vec2::new(844.0, 390.0),
            Self::Tablet => Vec2::new(834.0, 1112.0),
            Self::TabletLandscape => Vec2::new(1112.0, 834.0),
            Self::Laptop => Vec2::new(1440.0, 900.0),
        })
    }

    pub fn input_class(self, native: InputClass) -> InputClass {
        match self {
            Self::Native => native,
            Self::Phone | Self::PhoneLandscape | Self::Tablet | Self::TabletLandscape => {
                InputClass::Touch
            }
            Self::Laptop => InputClass::Pointer,
        }
    }
}

/// Map a window-space cursor into preview layout space (origin at letterbox,
/// size = preset logical size). Returns `None` if outside the letterbox.
pub fn translate_cursor(
    preset: PreviewPreset,
    window_area: Rect,
    cursor: Vec2,
) -> Option<Vec2> {
    let Some(logical) = preset.size() else {
        return Some(cursor);
    };
    let box_rect = letterbox_rect(window_area, logical);
    if !box_rect.contains(cursor) {
        return None;
    }
    let sx = logical.x / box_rect.size.x.max(1.0);
    let sy = logical.y / box_rect.size.y.max(1.0);
    Some(Vec2::new(
        (cursor.x - box_rect.origin.x) * sx,
        (cursor.y - box_rect.origin.y) * sy,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_letterbox_centered() {
        let outer = Rect::from_xywh(0.0, 0.0, 1440.0, 900.0);
        let r = letterbox_rect(outer, Vec2::new(390.0, 844.0));
        assert!((r.size.x - 390.0).abs() < 0.1);
        assert!((r.size.y - 844.0).abs() < 0.1);
        assert!((r.origin.x - (1440.0 - 390.0) * 0.5).abs() < 0.1);
    }

    #[test]
    fn translate_maps_into_preset_space() {
        let outer = Rect::from_xywh(0.0, 0.0, 1440.0, 900.0);
        let box_r = letterbox_rect(outer, Vec2::new(390.0, 844.0));
        let mapped = translate_cursor(
            PreviewPreset::Phone,
            outer,
            Vec2::new(box_r.origin.x, box_r.origin.y),
        );
        assert_eq!(mapped, Some(Vec2::new(0.0, 0.0)));
    }
}
