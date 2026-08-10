//! F10 debug overlay — particle tree readout of viewport + ResolveReport.

use crate::workspace::app_shell::AppShell;
use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};
use hyper_ui::{PageNode, ResolveReport, Visibility};

/// Build a translucent debug panel summarizing layout state.
pub fn build_overlay(shell: &AppShell) -> Particle {
    let mut lines = Vec::new();

    lines.push(format!(
        "viewport  logical {:.0}×{:.0}  physical {}×{}  scale {:.2}",
        shell.window_area.size.x,
        shell.window_area.size.y,
        shell.physical_width,
        shell.physical_height,
        shell.scale_factor
    ));
    lines.push(format!(
        "class  {:?}  input {:?}  preview {}",
        shell.size_class,
        shell.input_class,
        shell.preview.label()
    ));
    lines.push(format!(
        "focus  {}",
        shell
            .focus
            .chain
            .iter()
            .map(|id| id.0.to_string())
            .collect::<Vec<_>>()
            .join(" → ")
    ));

    if let Some(ws) = shell.active().and_then(|w| w.structural()) {
        let leaves = ws.page_tree.leaves();
        let ids: Vec<_> = leaves
            .iter()
            .map(|p| PageNode::container_id(p.id))
            .collect();
        lines.push("--- pages ---".into());
        for (i, leaf) in leaves.iter().enumerate() {
            let dist = shell.focus.distance(i, &ids);
            let override_on = ws
                .page_overrides
                .get(leaf.state.id, shell.size_class)
                .is_some();
            lines.push(format!(
                "  {}  intent={:?} resolved={:?} dist={} min={:.0} ideal={:.0} px={:.0}{}",
                leaf.state.label,
                leaf.state.intent,
                leaf.state.resolved(),
                dist,
                leaf.state.extent.min,
                leaf.state.extent.ideal,
                leaf.state.rect().size.x,
                if override_on { "  [override]" } else { "" }
            ));
        }
    }

    lines.push("--- ResolveReport ---".into());
    append_report(&mut lines, &shell.last_report);

    let children: Vec<_> = lines
        .into_iter()
        .map(|line| Particle::Source(SourceParticle::muted(line)))
        .collect();

    Particle::Surface(
        SurfaceParticle::new([0.05, 0.06, 0.08, 0.92])
            .with_padding(8.0)
            .with_radius(0.0)
            .with_border([0.35, 0.55, 0.75, 1.0], 1.0)
            .with_child(Particle::Stack(
                StackParticle::column(children).with_gap(2.0),
            )),
    )
}

fn append_report(lines: &mut Vec<String>, report: &ResolveReport) {
    if report.demotions.is_empty() && report.promotions.is_empty() {
        lines.push("  (no demotions / promotions)".into());
    }
    for (id, from, to) in &report.demotions {
        lines.push(format!(
            "  demote {}  {} → {}",
            id.0,
            vis_short(*from),
            vis_short(*to)
        ));
    }
    for (id, from, to) in &report.promotions {
        lines.push(format!(
            "  promote {}  {} → {}",
            id.0,
            vis_short(*from),
            vis_short(*to)
        ));
    }
    if report.scroll_extent > 0.0 {
        lines.push(format!("  scroll_extent {:.0}", report.scroll_extent));
    }
    if report.underflowed {
        lines.push("  UNDERFLOW".into());
    }
}

fn vis_short(v: Visibility) -> &'static str {
    match v {
        Visibility::Shown => "Shown",
        Visibility::Collapsed => "Collapsed",
        Visibility::Hidden => "Hidden",
    }
}
