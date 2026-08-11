//! Structural page IO assignments and empty-page helper.
//!
//! Page/pod *construction* comes from [`crate::workspace::seed::STRUCTURAL`] via
//! [`crate::workspace::from_seed`]. This module only maps seed IO labels → [`IoKind`]
//! and provides the empty-page assignment used after a split.

use super::io_kind::IoKind;
use crate::workspace::seed::{PageSeed, STRUCTURAL};
use hyper_ui::{PageId, PodId};
use std::collections::HashMap;

/// Build `(page_id → [(pod_id, IoKind)])` from Structural page seeds.
pub fn page_ios_from_seeds(pages: &[PageSeed]) -> HashMap<PageId, Vec<(PodId, IoKind)>> {
    let mut map = HashMap::new();
    for (i, page) in pages.iter().enumerate() {
        let page_id = PageId(i as u32);
        let mut ios = Vec::with_capacity(page.pods.len());
        for (j, pod) in page.pods.iter().enumerate() {
            let kind = pod
                .ios
                .first()
                .map(|io| io_kind_from_label(io.label))
                .unwrap_or(IoKind::Empty);
            ios.push((PodId(j as u32), kind));
        }
        map.insert(page_id, ios);
    }
    map
}

/// Initial Structural IO map — mirrors [`STRUCTURAL`] pages.
pub fn initial_page_ios() -> HashMap<PageId, Vec<(PodId, IoKind)>> {
    page_ios_from_seeds(STRUCTURAL.pages)
}

/// Empty page assignment used after a split.
pub fn empty_page_ios() -> Vec<(PodId, IoKind)> {
    vec![(PodId(0), IoKind::Empty)]
}

fn io_kind_from_label(label: &str) -> IoKind {
    match label {
        "WallList" => IoKind::WallList,
        "WallSummary" => IoKind::WallSummary,
        "InputForm" => IoKind::InputForm,
        "WallView" => IoKind::WallView,
        "ResultsTable" => IoKind::ResultsTable,
        "Status" => IoKind::Status,
        _ => IoKind::Empty,
    }
}
