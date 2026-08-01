use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::ParticleId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerState {
    Idle,
    Hover,
    Active,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct TriggerParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub label: String,
    pub state: TriggerState,
    pub primary: bool,
}

impl TriggerParticle {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            label: label.into(),
            state: TriggerState::Idle,
            primary: false,
        }
    }

    pub fn primary(label: impl Into<String>) -> Self {
        Self {
            primary: true,
            ..Self::new(label)
        }
    }

    pub fn measure(&self, _available: Vec2) -> Vec2 {
        let char_w = 14.0 * 0.55;
        let text_w = self.label.chars().count() as f32 * char_w;
        Vec2::new(text_w + 24.0, 36.0)
    }

    pub fn color(&self) -> [f32; 4] {
        if self.state == TriggerState::Disabled {
            return [0.25, 0.26, 0.28, 1.0];
        }
        let base = if self.primary {
            [0.18, 0.45, 0.85, 1.0]
        } else {
            [0.22, 0.24, 0.28, 1.0]
        };
        match self.state {
            TriggerState::Hover => [
                (base[0] + 0.08_f32).min(1.0),
                (base[1] + 0.08_f32).min(1.0),
                (base[2] + 0.08_f32).min(1.0),
                1.0,
            ],
            TriggerState::Active => [
                (base[0] - 0.06_f32).max(0.0),
                (base[1] - 0.06_f32).max(0.0),
                (base[2] - 0.06_f32).max(0.0),
                1.0,
            ],
            _ => base,
        }
    }
}
