use super::super::EdgeInstance;
use super::EdgeDrawCmd;

impl EdgeDrawCmd {
    pub fn to_instance(&self) -> EdgeInstance {
        EdgeInstance {
            p0: self.p0,
            p1: self.p1,
            p2: self.p2,
            p3: self.p3,
            color: self.color,
            width: self.width,
            edge_kind: self.edge_kind as u32,
            arrow: u32::from(self.arrow),
            _pad: 0,
        }
    }
}
