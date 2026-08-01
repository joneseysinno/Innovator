use crate::hyper_node::HyperNode;
use crate::prop_value::PropValue;

pub fn get_prop<'a>(node: &'a (impl HyperNode + ?Sized), key: &str) -> Option<&'a PropValue> {
    node.props().get(key)
}
