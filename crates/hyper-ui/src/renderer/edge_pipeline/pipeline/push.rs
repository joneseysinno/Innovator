use super::super::EdgeDrawCmd;
use super::EdgePipeline;

impl EdgePipeline {
    pub fn push(&mut self, cmd: &EdgeDrawCmd) {
        self.instances.push(cmd.to_instance());
    }
}
