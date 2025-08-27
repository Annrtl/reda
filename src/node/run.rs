use crate::{graph::Graph, node::Node, node_data::NodeData, node_field::NodeField};

#[derive(Debug, Clone)]
pub struct Run {
    pub name: String,
    pub target: Box<NodeField>,
}

impl Run {
    pub fn execute(self) -> Result<NodeData, String> {
        match *self.target {
            NodeField::NodeRef(node) => node.execute(),
            _ => {
                return Err(format!(
                    "Error, Run target is not a Node Reference: Node {}",
                    self.name
                ));
            }
        }
    }

    pub fn resolve(&mut self, graph: Graph) -> Result<(), String> {
        // Look for unresolved node and get its name else result
        let name = match &(*self.target) {
            NodeField::NodeRef(Node::Unresolved(n)) => n,
            _ => return Ok(()),
        };

        // Find the corresponding node
        let nodes = graph
            .nodes
            .iter()
            .filter(|n| (*n).clone().get_name() == *name)
            .map(|n| n.clone())
            .collect::<Vec<Node>>();

        // Get the node (shoud be unique)
        let Some(node) = nodes.first() else {
            return Err(format!("Node {} not found during graph resolution", name));
        };

        // Update the field with the node itself
        self.target = Box::new(NodeField::NodeRef(node.clone()));

        Ok(())
    }
}
