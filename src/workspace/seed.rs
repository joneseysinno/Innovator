//! First-run workspace seeds — write-once authoring data, never stored on containers.

use hyper_ui::container::{Extent, Visibility};

/// One stub IO slot inside a pod (no ContainerState).
///
/// For Structural, labels are mapped to explicit UI template identifiers at construction.
/// For placeholders, it is the stub display label.
#[derive(Debug, Clone, Copy)]
pub struct IoSeed {
    pub label: &'static str,
}

/// Pod placeholder — label, extent demand, and one or more stub IO.
///
/// `icon` maps to live [`hyper_ui::Pod::nav_icon`] when non-empty.
#[derive(Debug, Clone, Copy)]
pub struct PodSeed {
    pub label: &'static str,
    pub icon: &'static str,
    pub extent: Extent,
    pub ios: &'static [IoSeed],
}

/// Page placeholder — label and ordered pods.
#[derive(Debug, Clone, Copy)]
pub struct PageSeed {
    pub label: &'static str,
    pub icon: &'static str,
    pub extent: Extent,
    pub pods: &'static [PodSeed],
    /// When true, page gets a Custom header bar (Structural Analysis).
    pub custom_header: bool,
}

/// Workspace placeholder — consumed once at first run.
#[derive(Debug, Clone, Copy)]
pub struct WorkspaceSeed {
    /// Stable open-id for Home launcher / OpenWorkspace (not stored as "kind").
    pub open_id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub intent: Visibility,
    pub pages: &'static [PageSeed],
}

// ── Shared extents ──────────────────────────────────────────────────────────

const PAGE_DOCS: Extent = Extent::new(280.0, 600.0, 1.0);
const PAGE_VIEWER: Extent = Extent::new(320.0, 800.0, 1.0);
const PAGE_OVERVIEW: Extent = Extent::new(400.0, 960.0, 1.0);
const POD_DOC: Extent = Extent::new(120.0, 280.0, 1.0);
const POD_VIEW: Extent = Extent::new(160.0, 420.0, 1.5);
const POD_GEOMETRY: Extent = Extent::new(200.0, 420.0, 1.2);

/// Pod weight extents — `weight` is the PodList height flex (matches Path A ratios).
const POD_WEIGHT_REF: f32 = 480.0;
const fn pod_weight(weight: f32) -> Extent {
    Extent::new(80.0, weight * POD_WEIGHT_REF, weight)
}

const STUB_ONE: &[IoSeed] = &[IoSeed { label: "Stub" }];
const STUB_GEOMETRY: &[IoSeed] = &[
    IoSeed { label: "Length" },
    IoSeed { label: "Width" },
    IoSeed { label: "Height" },
];

// ── Home ────────────────────────────────────────────────────────────────────

const HOME_LAUNCHER_PODS: &[PodSeed] = &[PodSeed {
    label: "Launcher",
    icon: "H",
    extent: POD_DOC,
    ios: &[IoSeed { label: "Open workspace" }],
}];
const HOME_RECENTS_PODS: &[PodSeed] = &[PodSeed {
    label: "Recents",
    icon: "H",
    extent: POD_DOC,
    ios: &[IoSeed { label: "Recent projects" }],
}];
const HOME_PAGES: &[PageSeed] = &[
    PageSeed {
        label: "Launcher",
        icon: "H",
        extent: PAGE_OVERVIEW,
        pods: HOME_LAUNCHER_PODS,
        custom_header: false,
    },
    PageSeed {
        label: "Recents",
        icon: "H",
        extent: PAGE_DOCS,
        pods: HOME_RECENTS_PODS,
        custom_header: false,
    },
];

pub const HOME: WorkspaceSeed = WorkspaceSeed {
    open_id: "home",
    label: "Home",
    icon: "H",
    intent: Visibility::Shown,
    pages: HOME_PAGES,
};

// ── Structural (Navigation · Analysis · Results) ────────────────────────────

const STRUCTURAL_NAV_PODS: &[PodSeed] = &[
    PodSeed {
        label: "Wall List",
        icon: "≡",
        extent: pod_weight(0.35),
        ios: &[IoSeed { label: "WallList" }],
    },
    PodSeed {
        label: "Summary",
        icon: "▣",
        extent: pod_weight(0.65),
        ios: &[IoSeed { label: "WallSummary" }],
    },
];
const STRUCTURAL_ANALYSIS_PODS: &[PodSeed] = &[
    PodSeed {
        label: "Input",
        icon: "▤",
        extent: pod_weight(0.30),
        ios: &[IoSeed { label: "InputForm" }],
    },
    PodSeed {
        label: "Wall View",
        icon: "▥",
        extent: pod_weight(0.70),
        ios: &[IoSeed { label: "WallView" }],
    },
];
const STRUCTURAL_RESULTS_PODS: &[PodSeed] = &[
    PodSeed {
        label: "Results",
        icon: "▦",
        extent: pod_weight(0.70),
        ios: &[IoSeed { label: "ResultsTable" }],
    },
    PodSeed {
        label: "Status",
        icon: "▧",
        extent: pod_weight(0.30),
        ios: &[IoSeed { label: "Status" }],
    },
];

const STRUCTURAL_PAGES: &[PageSeed] = &[
    PageSeed {
        label: "Navigation",
        icon: "N",
        extent: Extent::new(280.0, 360.0, 0.0),
        pods: STRUCTURAL_NAV_PODS,
        custom_header: true,
    },
    PageSeed {
        label: "Analysis",
        icon: "A",
        extent: Extent::new(400.0, 800.0, 1.0),
        pods: STRUCTURAL_ANALYSIS_PODS,
        custom_header: true,
    },
    PageSeed {
        label: "Results",
        icon: "R",
        extent: Extent::new(320.0, 560.0, 1.0),
        pods: STRUCTURAL_RESULTS_PODS,
        custom_header: true,
    },
];

pub const STRUCTURAL: WorkspaceSeed = WorkspaceSeed {
    open_id: "structural_analysis",
    label: "Structural",
    icon: "S",
    intent: Visibility::Hidden,
    pages: STRUCTURAL_PAGES,
};

// ── Project Management ──────────────────────────────────────────────────────

const PM_PODS: &[PodSeed] = &[PodSeed {
    label: "Overview",
    icon: "P",
    extent: POD_VIEW,
    ios: STUB_ONE,
}];
const PM_PAGES: &[PageSeed] = &[PageSeed {
    label: "Overview",
    icon: "P",
    extent: PAGE_OVERVIEW,
    pods: PM_PODS,
    custom_header: false,
}];

pub const PROJECT_MANAGEMENT: WorkspaceSeed = WorkspaceSeed {
    open_id: "project_management",
    label: "Project Management",
    icon: "P",
    intent: Visibility::Hidden,
    pages: PM_PAGES,
};

// ── Engineering (Geometry pod holds three stub IO) ──────────────────────────

const ENG_DOCS_PODS: &[PodSeed] = &[PodSeed {
    label: "Documents",
    icon: "E",
    extent: POD_DOC,
    ios: STUB_ONE,
}];
const ENG_VIEW_PODS: &[PodSeed] = &[
    PodSeed {
        label: "Geometry",
        icon: "E",
        extent: POD_GEOMETRY,
        ios: STUB_GEOMETRY,
    },
    PodSeed {
        label: "Viewer",
        icon: "E",
        extent: POD_VIEW,
        ios: STUB_ONE,
    },
];
const ENG_PAGES: &[PageSeed] = &[
    PageSeed {
        label: "Documents",
        icon: "E",
        extent: PAGE_DOCS,
        pods: ENG_DOCS_PODS,
        custom_header: false,
    },
    PageSeed {
        label: "Viewer",
        icon: "E",
        extent: PAGE_VIEWER,
        pods: ENG_VIEW_PODS,
        custom_header: false,
    },
];

pub const ENGINEERING: WorkspaceSeed = WorkspaceSeed {
    open_id: "engineering",
    label: "Engineering",
    icon: "E",
    intent: Visibility::Hidden,
    pages: ENG_PAGES,
};

// ── Drafting ────────────────────────────────────────────────────────────────

const DRAFT_PAGES: &[PageSeed] = &[
    PageSeed {
        label: "Templates",
        icon: "D",
        extent: PAGE_DOCS,
        pods: &[PodSeed {
            label: "Templates",
            icon: "D",
            extent: POD_DOC,
            ios: STUB_ONE,
        }],
        custom_header: false,
    },
    PageSeed {
        label: "Viewer",
        icon: "D",
        extent: PAGE_VIEWER,
        pods: &[PodSeed {
            label: "Viewer",
            icon: "D",
            extent: POD_VIEW,
            ios: STUB_ONE,
        }],
        custom_header: false,
    },
];

pub const DRAFTING: WorkspaceSeed = WorkspaceSeed {
    open_id: "drafting",
    label: "Drafting",
    icon: "D",
    intent: Visibility::Hidden,
    pages: DRAFT_PAGES,
};

// ── Crane & Construction ────────────────────────────────────────────────────

const CRANE_PAGES: &[PageSeed] = &[
    PageSeed {
        label: "Documents",
        icon: "C",
        extent: PAGE_DOCS,
        pods: &[PodSeed {
            label: "Documents",
            icon: "C",
            extent: POD_DOC,
            ios: STUB_ONE,
        }],
        custom_header: false,
    },
    PageSeed {
        label: "Viewer",
        icon: "C",
        extent: PAGE_VIEWER,
        pods: &[PodSeed {
            label: "Viewer",
            icon: "C",
            extent: POD_VIEW,
            ios: STUB_ONE,
        }],
        custom_header: false,
    },
];

pub const CRANE: WorkspaceSeed = WorkspaceSeed {
    open_id: "crane_construction",
    label: "Crane & Construction",
    icon: "C",
    intent: Visibility::Hidden,
    pages: CRANE_PAGES,
};

// ── HR ──────────────────────────────────────────────────────────────────────

const HR_PAGES: &[PageSeed] = &[PageSeed {
    label: "Documents",
    icon: "R",
    extent: PAGE_DOCS,
    pods: &[PodSeed {
        label: "Documents",
        icon: "R",
        extent: POD_DOC,
        ios: STUB_ONE,
    }],
    custom_header: false,
}];

pub const HR: WorkspaceSeed = WorkspaceSeed {
    open_id: "hr",
    label: "HR",
    icon: "R",
    intent: Visibility::Hidden,
    pages: HR_PAGES,
};

// ── Accounting ──────────────────────────────────────────────────────────────

const ACCOUNTING_PAGES: &[PageSeed] = &[PageSeed {
    label: "Documents",
    icon: "A",
    extent: PAGE_DOCS,
    pods: &[PodSeed {
        label: "Documents",
        icon: "A",
        extent: POD_DOC,
        ios: STUB_ONE,
    }],
    custom_header: false,
}];

pub const ACCOUNTING: WorkspaceSeed = WorkspaceSeed {
    open_id: "accounting",
    label: "Accounting",
    icon: "A",
    intent: Visibility::Hidden,
    pages: ACCOUNTING_PAGES,
};

// ── Devtools Graph (F11) ────────────────────────────────────────────────────

const DEVTOOLS_GRAPH_PODS: &[PodSeed] = &[
    PodSeed {
        label: "Graph",
        icon: "G",
        extent: pod_weight(0.70),
        ios: &[IoSeed { label: "GraphView" }],
    },
    PodSeed {
        label: "Inspector",
        icon: "I",
        extent: pod_weight(0.30),
        ios: &[IoSeed { label: "Inspector" }],
    },
];
const DEVTOOLS_GRAPH_PAGES: &[PageSeed] = &[PageSeed {
    label: "Graph View",
    icon: "G",
    extent: PAGE_VIEWER,
    pods: DEVTOOLS_GRAPH_PODS,
    custom_header: false,
}];

pub const DEVTOOLS_GRAPH: WorkspaceSeed = WorkspaceSeed {
    open_id: "devtools_graph",
    label: "Graph",
    icon: "G",
    intent: Visibility::Hidden,
    pages: DEVTOOLS_GRAPH_PAGES,
};

/// All workspace seeds in tab / launcher order. Home first.
pub const ALL: &[WorkspaceSeed] = &[
    HOME,
    STRUCTURAL,
    PROJECT_MANAGEMENT,
    ENGINEERING,
    DRAFTING,
    CRANE,
    HR,
    ACCOUNTING,
    DEVTOOLS_GRAPH,
];

/// Non-Home seeds — Home launcher buttons.
pub const LAUNCHABLE: &[WorkspaceSeed] = &[
    STRUCTURAL,
    PROJECT_MANAGEMENT,
    ENGINEERING,
    DRAFTING,
    CRANE,
    HR,
    ACCOUNTING,
];
