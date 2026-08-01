pub mod get_prop;
pub mod set_prop;

use crate::ids::NodeId;
use crate::prop_value::PropValue;
use crate::space_class::SpaceClass;
use std::collections::BTreeMap;

/// Core trait for anything addressable in the spatial hypergraph.
pub trait HyperNode {
    fn id(&self) -> NodeId;
    fn space_class(&self) -> SpaceClass;
    fn label(&self) -> &str;
    fn world_pos(&self) -> [f64; 2];
    fn props(&self) -> &BTreeMap<String, PropValue>;
    fn props_mut(&mut self) -> &mut BTreeMap<String, PropValue>;

    fn get_prop(&self, key: &str) -> Option<&PropValue> {
        get_prop::get_prop(self, key)
    }

    fn set_prop(&mut self, key: impl Into<String>, value: PropValue) {
        set_prop::set_prop(self, key, value)
    }
}
