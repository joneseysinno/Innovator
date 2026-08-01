use super::ResultsTableIO;
use crate::engine::CheckResult;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, ViewParticle,
};

/// Build ResultsTableIO from parsed checks (empty state when none).
pub fn build_results_table(checks: &[CheckResult]) -> ResultsTableIO {
    let title = SourceParticle::new("Results").with_weight(500);
    let mut children = vec![Particle::Source(title)];

    if checks.is_empty() {
        children.push(Particle::Source(SourceParticle::secondary(
            "Run analysis to see checks",
        )));
    } else {
        children.push(header_row());
        for check in checks {
            children.push(check_row(check));
        }
    }

    let body = StackParticle::column(children).with_gap(4.0);
    let surface = SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
        .with_padding(10.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(body));

    let mut view = ViewParticle::new("results_table");
    view.child = Some(Box::new(Particle::Surface(surface)));

    ResultsTableIO {
        particle: Particle::View(view),
    }
}

fn header_row() -> Particle {
    let row = StackParticle::row(vec![
        cell("Check", true),
        cell("Demand", true),
        cell("Capacity", true),
        cell("Ratio", true),
        cell("Status", true),
    ])
    .with_gap(8.0);
    Particle::Stack(row)
}

fn check_row(check: &CheckResult) -> Particle {
    let (badge_text, badge_color) = if check.informational {
        ("info", [0.35, 0.40, 0.48, 1.0])
    } else if check.pass {
        ("PASS", [0.18, 0.42, 0.28, 1.0])
    } else {
        ("FAIL", [0.50, 0.20, 0.20, 1.0])
    };

    let demand = if check.informational && check.unit.len() > 8 {
        check.unit.clone()
    } else {
        format!("{:.3}", check.demand)
    };
    let capacity = if check.informational {
        "—".into()
    } else {
        format!("{:.3}", check.capacity)
    };
    let ratio = if check.informational {
        "—".into()
    } else {
        format!("{:.3}", check.ratio)
    };

    let badge = Particle::Surface(
        SurfaceParticle::new(badge_color)
            .with_padding(4.0)
            .with_radius(0.0)
            .with_child(Particle::Source(SourceParticle::new(badge_text))),
    );

    let row = StackParticle::row(vec![
        cell(&check.name, false),
        cell(&demand, false),
        cell(&capacity, false),
        cell(&ratio, false),
        badge,
    ])
    .with_gap(8.0);

    Particle::Surface(
        SurfaceParticle::new([0.14, 0.15, 0.18, 1.0])
            .with_padding(6.0)
            .with_radius(0.0)
            .with_border([0.24, 0.26, 0.30, 1.0], 1.0)
            .with_child(Particle::Stack(row)),
    )
}

fn cell(text: &str, header: bool) -> Particle {
    let src = if header {
        SourceParticle::muted(text)
    } else {
        SourceParticle::secondary(text)
    };
    Particle::Source(src)
}
