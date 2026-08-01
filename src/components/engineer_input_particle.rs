use crate::components::engineer_input;
use hyper_ui::Particle;

pub fn engineer_input_particle(label: &str, value: f64, unit: &str) -> Particle {
    engineer_input(label, value, unit).into_particle()
}
