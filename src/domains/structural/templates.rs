//! Structural page/pod template assignments derived from authored seeds.

use super::template_ids::*;
use crate::workspace::seed::{PageSeed, STRUCTURAL};
use hyper_ui::{PageId, PodId, TemplateId};
use std::collections::HashMap;

pub fn page_templates_from_seeds(pages: &[PageSeed]) -> HashMap<PageId, TemplateId> {
    pages
        .iter()
        .enumerate()
        .map(|(i, page)| (PageId(i as u32), page_template_from_label(page.label)))
        .collect()
}

pub fn pod_templates_from_seeds(pages: &[PageSeed]) -> HashMap<(PageId, PodId), TemplateId> {
    let mut templates = HashMap::new();
    for (i, page) in pages.iter().enumerate() {
        let page_id = PageId(i as u32);
        for (j, pod) in page.pods.iter().enumerate() {
            templates.insert(
                (page_id, PodId(j as u32)),
                pod_template_from_label(pod.label),
            );
        }
    }
    templates
}

pub fn initial_page_templates() -> HashMap<PageId, TemplateId> {
    page_templates_from_seeds(STRUCTURAL.pages)
}

pub fn initial_pod_templates() -> HashMap<(PageId, PodId), TemplateId> {
    pod_templates_from_seeds(STRUCTURAL.pages)
}

pub fn template_from_str(id: &str) -> TemplateId {
    match id {
        "navigation" => NAVIGATION,
        "analysis" => ANALYSIS,
        "results" => RESULTS,
        "wall_list" => WALL_LIST,
        "wall_summary" => WALL_SUMMARY,
        "input_form" => INPUT_FORM,
        "wall_view" => WALL_VIEW,
        "results_table" => RESULTS_TABLE,
        "status" => STATUS,
        _ => GENERIC,
    }
}

fn page_template_from_label(label: &str) -> TemplateId {
    match label {
        "Navigation" => NAVIGATION,
        "Analysis" => ANALYSIS,
        "Results" => RESULTS,
        _ => GENERIC,
    }
}

fn pod_template_from_label(label: &str) -> TemplateId {
    match label {
        "Wall List" => WALL_LIST,
        "Summary" => WALL_SUMMARY,
        "Input" => INPUT_FORM,
        "Wall View" => WALL_VIEW,
        "Results" => RESULTS_TABLE,
        "Status" => STATUS,
        _ => GENERIC,
    }
}
