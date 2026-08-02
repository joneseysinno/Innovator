use crate::workspace::analysis::PageSignal;
use hyper_ui::particles::{
    Particle, ParticleId, StackParticle, SurfaceParticle, TriggerParticle,
};
use hyper_ui::{PageSeamId, PageSide, SeamDirection, Vec2};
use std::collections::HashMap;

/// Pending right-click menu on a page seam.
#[derive(Debug, Clone)]
pub struct PageContextMenu {
    pub seam_id: PageSeamId,
    pub cursor: Vec2,
    pub direction: SeamDirection,
    /// Which side of the seam the cursor was on (for split target).
    pub side: PageSide,
}

/// Built context menu with trigger → signal map.
pub struct PageContextMenuIo {
    pub particle: Particle,
    pub triggers: HashMap<ParticleId, PageSignal>,
}

pub fn build_page_context_menu(menu: &PageContextMenu) -> PageContextMenuIo {
    let (merge_first, merge_second) = match menu.direction {
        SeamDirection::Vertical => ("Merge ←", "Merge →"),
        SeamDirection::Horizontal => ("Merge ↑", "Merge ↓"),
    };

    let items: [(&str, PageSignal); 5] = [
        (
            "Split vertical",
            PageSignal::Split {
                seam_id: menu.seam_id,
                direction: SeamDirection::Vertical,
                side: menu.side,
            },
        ),
        (
            "Split horizontal",
            PageSignal::Split {
                seam_id: menu.seam_id,
                direction: SeamDirection::Horizontal,
                side: menu.side,
            },
        ),
        (
            merge_first,
            PageSignal::Merge {
                seam_id: menu.seam_id,
                keep: PageSide::First,
            },
        ),
        (
            merge_second,
            PageSignal::Merge {
                seam_id: menu.seam_id,
                keep: PageSide::Second,
            },
        ),
        (
            "Reset 50/50",
            PageSignal::ResetRatio {
                seam_id: menu.seam_id,
            },
        ),
    ];

    let mut triggers = HashMap::new();
    let mut children = Vec::new();
    for (label, signal) in items {
        let t = TriggerParticle::new(label);
        triggers.insert(t.id, signal);
        children.push(Particle::Trigger(t));
    }

    let column = StackParticle::column(children).with_gap(2.0);
    let particle = Particle::Surface(
        SurfaceParticle::new([0.18, 0.19, 0.22, 1.0])
            .with_padding(6.0)
            .with_radius(4.0)
            .with_border([0.40, 0.42, 0.48, 1.0], 1.0)
            .with_child(Particle::Stack(column)),
    );

    PageContextMenuIo { particle, triggers }
}
