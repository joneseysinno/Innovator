use crate::hyper_node::HyperNode;
use crate::prop_value::PropValue;

pub fn set_prop(node: &mut (impl HyperNode + ?Sized), key: impl Into<String>, value: PropValue) {
    node.props_mut().insert(key.into(), value);
}
