use super::ScreenClass;

impl ScreenClass {
    pub fn from_width(width: u32) -> Self {
        if width < 700 {
            Self::Mobile
        } else if width < 1100 {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }
}
