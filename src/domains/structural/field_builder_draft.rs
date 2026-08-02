use super::action::AnalysisAction;

/// Draft values while the inline FieldBuilderIO is open.
#[derive(Debug, Clone)]
pub struct FieldBuilderDraft {
    pub label: String,
    pub initial: f64,
    pub unit: String,
    pub kind: CustomFieldKind,
    pub min: f64,
    pub max: f64,
}

impl Default for FieldBuilderDraft {
    fn default() -> Self {
        Self {
            label: String::new(),
            initial: 0.0,
            unit: String::new(),
            kind: CustomFieldKind::Number,
            min: 0.0,
            max: 100.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CustomFieldKind {
    Number,
    Text,
    Bool,
}

impl CustomFieldKind {
    pub fn from_action(action: AnalysisAction) -> Option<Self> {
        match action {
            AnalysisAction::FieldKindNumber => Some(Self::Number),
            AnalysisAction::FieldKindText => Some(Self::Text),
            AnalysisAction::FieldKindBool => Some(Self::Bool),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Number => "Number",
            Self::Text => "Text",
            Self::Bool => "Bool",
        }
    }
}

/// Which FieldBuilder field a ParticleId maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuilderFieldSlot {
    Label,
    Initial,
    Unit,
    Min,
    Max,
}
