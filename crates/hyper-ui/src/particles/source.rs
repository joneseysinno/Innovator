use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::ParticleId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceStyle {
    Primary,
    Secondary,
    Muted,
}

#[derive(Debug, Clone)]
pub struct SourceParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub text: String,
    pub style: SourceStyle,
    pub font_size: f32,
    pub weight: u16,
}

impl SourceParticle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            text: text.into(),
            style: SourceStyle::Primary,
            font_size: 14.0,
            weight: 400,
        }
    }

    pub fn secondary(text: impl Into<String>) -> Self {
        Self {
            style: SourceStyle::Secondary,
            font_size: 12.0,
            ..Self::new(text)
        }
    }

    pub fn muted(text: impl Into<String>) -> Self {
        Self {
            style: SourceStyle::Muted,
            font_size: 12.0,
            ..Self::new(text)
        }
    }

    pub fn with_weight(mut self, weight: u16) -> Self {
        self.weight = weight;
        self
    }

    pub fn color(&self) -> [f32; 4] {
        match self.style {
            SourceStyle::Primary => [0.92, 0.93, 0.95, 1.0],
            SourceStyle::Secondary => [0.70, 0.72, 0.76, 1.0],
            SourceStyle::Muted => [0.50, 0.52, 0.56, 1.0],
        }
    }

    pub fn measure(&self, _available: Vec2) -> Vec2 {
        // Approximate until glyphon measures; good enough for layout.
        let char_w = self.font_size * 0.55;
        let w = (self.text.chars().count() as f32 * char_w).max(8.0);
        let h = self.font_size * 1.35;
        Vec2::new(w, h)
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}
