use crate::workspace::size_class::SizeClass;

/// Standard form field definition.
#[derive(Debug, Clone, Copy)]
pub struct FieldDef {
    pub key: &'static str,
    pub label: &'static str,
    pub short: &'static str,
    pub unit: &'static str,
    pub is_u8: bool,
    pub is_text: bool,
}

pub const GEOMETRY: &[FieldDef] = &[
    FieldDef {
        key: "height",
        label: "Height",
        short: "H",
        unit: "ft",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "length",
        label: "Length",
        short: "L",
        unit: "ft",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "thickness",
        label: "Thickness",
        short: "t",
        unit: "in",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "clear_cover",
        label: "Clear cover",
        short: "cc",
        unit: "in",
        is_u8: false,
        is_text: false,
    },
];

pub const MATERIAL: &[FieldDef] = &[
    FieldDef {
        key: "fc",
        label: "f'c",
        short: "fc",
        unit: "psi",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "fy",
        label: "fy",
        short: "fy",
        unit: "psi",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "es",
        label: "Es",
        short: "Es",
        unit: "ksi",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "lambda",
        label: "λ",
        short: "λ",
        unit: "",
        is_u8: false,
        is_text: false,
    },
];

pub const REINFORCEMENT: &[FieldDef] = &[
    FieldDef {
        key: "vert_bar_size",
        label: "Vert bar #",
        short: "Vb",
        unit: "",
        is_u8: true,
        is_text: false,
    },
    FieldDef {
        key: "vert_spacing",
        label: "Vert spacing",
        short: "Vs",
        unit: "in",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "horiz_bar_size",
        label: "Horiz bar #",
        short: "Hb",
        unit: "",
        is_u8: true,
        is_text: false,
    },
    FieldDef {
        key: "horiz_spacing",
        label: "Horiz spacing",
        short: "Hs",
        unit: "in",
        is_u8: false,
        is_text: false,
    },
];

pub const LOADING: &[FieldDef] = &[
    FieldDef {
        key: "pu",
        label: "Pu",
        short: "Pu",
        unit: "kips",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "vu",
        label: "Vu",
        short: "Vu",
        unit: "kips",
        is_u8: false,
        is_text: false,
    },
    FieldDef {
        key: "mu",
        label: "Mu",
        short: "Mu",
        unit: "kip-ft",
        is_u8: false,
        is_text: false,
    },
];

impl FieldDef {
    pub fn display_label(self, size: SizeClass) -> &'static str {
        if size.hide_labels() {
            ""
        } else if size.abbreviate() {
            self.short
        } else {
            self.label
        }
    }
}
