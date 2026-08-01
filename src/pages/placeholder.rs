use crate::workspace::page::Page;
use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};

/// Phase 1 placeholder fill for a page region (full IO arrives in later phases).
pub fn build_page_placeholder(page: Page) -> Particle {
    let title = SourceParticle::new(page.title()).with_weight(500);
    let hint = SourceParticle::secondary(match page {
        Page::Navigation => "Navigation · Phase 2",
        Page::Analysis => "Analysis · Phase 3",
        Page::Results => "Results · Phase 4",
    });

    let body = StackParticle::column(vec![Particle::Source(title), Particle::Source(hint)]).with_gap(8.0);

    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(12.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    )
}
