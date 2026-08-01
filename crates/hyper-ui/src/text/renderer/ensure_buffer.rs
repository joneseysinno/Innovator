use crate::text::TextKey;

use super::TextRenderer;

impl TextRenderer {
    pub(crate) fn ensure_buffer(&mut self, key: &TextKey) {
        if !self.cache.contains_key(key) {
            let buffer = self.make_buffer(key);
            self.cache.insert(key.clone(), buffer);
        }
    }
}
