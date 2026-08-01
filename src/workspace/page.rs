/// Named spatial page region in the concrete wall workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Navigation,
    Analysis,
    Results,
}

impl Page {
    pub fn all() -> [Page; 3] {
        [Page::Navigation, Page::Analysis, Page::Results]
    }

    pub fn title(self) -> &'static str {
        match self {
            Page::Navigation => "Navigation",
            Page::Analysis => "Analysis",
            Page::Results => "Results",
        }
    }

    pub fn leaf_id(self) -> u32 {
        match self {
            Page::Navigation => 0,
            Page::Analysis => 1,
            Page::Results => 2,
        }
    }
}
