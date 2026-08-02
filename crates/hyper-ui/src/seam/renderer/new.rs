use super::SeamRenderer;

impl SeamRenderer {
    pub fn new() -> Self {
        Self {
            seams: Vec::new(),
            drag: None,
            last_click: None,
        }
    }
}
