use super::NodePipeline;

impl NodePipeline {
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }
}
