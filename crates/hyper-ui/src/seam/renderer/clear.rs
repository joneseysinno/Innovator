use super::SeamRenderer;

impl SeamRenderer {
    pub fn clear(&mut self) {
        self.seams.clear();
        self.pod_owners.clear();
        self.drag = None;
        self.last_click = None;
    }
}
