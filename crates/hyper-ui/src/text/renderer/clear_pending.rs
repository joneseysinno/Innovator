use super::TextRenderer;

impl TextRenderer {
    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }
}
