use super::AnalysisWorkspace;
use crate::pages::{build_analysis, build_navigation, build_results};
use crate::workspace::page::Page;
use hyper_ui::particles::{Particle, StackParticle, ViewParticle};

/// Page region particle for an Analysis workspace.
pub fn build_pages(ws: &mut AnalysisWorkspace) -> Particle {
    let mut children = Vec::with_capacity(3);
    for page in Page::all() {
        let particle = match page {
            Page::Navigation => build_navigation(ws),
            Page::Analysis => build_analysis(ws),
            Page::Results => build_results(ws),
        };
        children.push(particle);
    }

    let mut pages_view = ViewParticle::new("pages");
    pages_view.child = Some(Box::new(Particle::Stack(
        StackParticle::row(children).with_gap(0.0),
    )));
    Particle::View(pages_view)
}
