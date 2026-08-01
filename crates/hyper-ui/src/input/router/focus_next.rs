use crate::particles::{Particle, ParticleTree};

use super::InputRouter;
use crate::input::UiEvent;

impl InputRouter {
    pub(crate) fn focus_next(&mut self, tree: &mut ParticleTree, reverse: bool) -> Vec<UiEvent> {
        let mut out = Vec::new();
        if self.tab_order.is_empty() {
            return out;
        }
        let from = self.focused;
        let next = match from.and_then(|id| self.tab_order.iter().position(|&x| x == id)) {
            Some(i) if reverse => {
                if i == 0 {
                    *self.tab_order.last().unwrap()
                } else {
                    self.tab_order[i - 1]
                }
            }
            Some(i) => self.tab_order[(i + 1) % self.tab_order.len()],
            None => self.tab_order[0],
        };
        self.blur_current(tree);
        if let Some(Particle::Field(f)) = tree.find_mut(next) {
            f.begin_edit();
            tree.mark_paint(next);
        }
        self.focused = Some(next);
        out.push(UiEvent::FocusChanged {
            from,
            to: Some(next),
        });
        out
    }
}
