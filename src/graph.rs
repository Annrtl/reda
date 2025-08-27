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

    pub fn resolve(&mut self) -> Result<(), String> {
        let graph = self.clone();

        for node in &mut self.nodes {
            node.resolve(graph.clone())?;
        }

        Ok(())
    }
}
