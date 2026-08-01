use super::TextRenderer;

impl TextRenderer {
    pub fn trim(&mut self) {
        self.atlas.trim();
        if self.cache.len() > 256 {
            self.cache.clear();
        }
        self.pending.clear();
    }
}
