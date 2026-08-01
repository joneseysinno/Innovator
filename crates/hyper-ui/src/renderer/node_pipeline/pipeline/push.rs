use super::super::NodeInstance;
use super::NodePipeline;

impl NodePipeline {
    pub fn push(&mut self, instance: NodeInstance) {
        self.instances.push(instance);
    }
}
