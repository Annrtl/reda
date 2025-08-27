use std::cell::RefCell;

use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

thread_local! {
    pub static NODES: RefCell<Graph> = RefCell::new(Graph::default());
}

impl Graph {
    pub fn default() -> Graph {
        Graph { nodes: vec![] }
    }

    pub fn get_node(&self, name: &str) -> Result<Node, String> {
        for node in &self.nodes {
            if node.clone().get_name() == name {
                return Ok(node.clone());
            }
        }
        return Err(format!("Failed to find node `{}` in the graph", name));
    }
}
