use crate::domains::structural::templates::{
    page_template_label, switchable_page_templates,
};
use crate::domains::structural::PageSignal;
use hyper_ui::particles::{
    Particle, ParticleId, StackParticle, SurfaceParticle, TriggerParticle,
};
use hyper_ui::{PageId, TemplateId, Vec2};
use std::collections::HashMap;

/// Pending editor-type dropdown on a page header.
#[derive(Debug, Clone)]
pub struct PageTemplateMenu {
    pub page_id: PageId,
    pub cursor: Vec2,
    pub current: TemplateId,
}

/// Built template menu with trigger → signal map.
pub struct PageTemplateMenuIo {
    pub particle: Particle,
    pub triggers: HashMap<ParticleId, PageSignal>,
}

pub fn build_page_template_menu(menu: &PageTemplateMenu) -> PageTemplateMenuIo {
    let mut triggers = HashMap::new();
    let mut children = Vec::new();
    for &template_id in switchable_page_templates() {
        let label = page_template_label(template_id);
        let text = if template_id == menu.current {
            format!("● {label}")
        } else {
            format!("  {label}")
        };
        let t = TriggerParticle::new(text);
        triggers.insert(
            t.id,
            PageSignal::SwitchTemplate {
                page_id: menu.page_id,
                template_id,
            },
        );
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

    PageTemplateMenuIo { particle, triggers }
}
