#[derive(Debug, Clone)]
pub enum NodeField {
    NodeRef(String),
    String(String),
}

impl NodeField {
    pub fn auto(name: &str) -> NodeField {
        if name.starts_with(":") {
            let node_name = name.replace(":", "");
            return NodeField::NodeRef(node_name);
        } else {
            return NodeField::String(name.to_string());
        }
    }
}
