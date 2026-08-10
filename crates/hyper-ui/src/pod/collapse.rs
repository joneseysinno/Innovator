use super::{PodId, PodList};

impl PodList {
    pub fn collapse(&mut self, id: PodId) {
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(true);
        }
    }

    pub fn expand(&mut self, id: PodId) {
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(false);
        }
    }

    pub fn toggle(&mut self, id: PodId) {
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(!pod.collapsed);
        }
    }
}
