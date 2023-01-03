use web_sys::HtmlTextAreaElement;
use yew::NodeRef;

pub fn node_str(node: &NodeRef) -> String {
    node.cast::<HtmlTextAreaElement>().unwrap().value()
}