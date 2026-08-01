use crate::hyper_node::HyperNode;
use crate::ids::NodeId;
use crate::node::Node;
use crate::prop_value::PropValue;
use crate::space_class::SpaceClass;
use std::collections::BTreeMap;

impl HyperNode for Node {
    fn id(&self) -> NodeId {
        self.id
    }
    fn space_class(&self) -> SpaceClass {
        self.space_class
    }
    fn label(&self) -> &str {
        &self.label
    }
    fn world_pos(&self) -> [f64; 2] {
        self.world_pos
    }
    fn props(&self) -> &BTreeMap<String, PropValue> {
        &self.props
    }
    fn props_mut(&mut self) -> &mut BTreeMap<String, PropValue> {
        &mut self.props
    }
}
