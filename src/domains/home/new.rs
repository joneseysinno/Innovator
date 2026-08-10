use super::workspace::HomeWorkspace;
use std::collections::HashMap;

impl HomeWorkspace {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }
}
