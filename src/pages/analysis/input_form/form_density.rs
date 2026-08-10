//! Form label density for Analysis InputForm (not viewport SizeClass).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FormDensity {
    #[default]
    Full,
    Compact,
    Minimal,
}

impl FormDensity {
    pub fn from_width(width: f32) -> Self {
        if width > 320.0 {
            Self::Full
        } else if width >= 200.0 {
            Self::Compact
        } else {
            Self::Minimal
        }
    }

    pub fn abbreviate(self) -> bool {
        matches!(self, Self::Compact | Self::Minimal)
    }

    pub fn hide_labels(self) -> bool {
        matches!(self, Self::Minimal)
    }
}
