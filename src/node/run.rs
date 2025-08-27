use crate::{graph::Graph, node_data::NodeData, node_field::NodeField};

#[derive(Debug, Clone)]
pub struct Run {
    pub name: String,
    pub target: NodeField,
}

impl Run {
    /// Execute the node
    pub fn execute(self, graph: &Graph) -> Result<NodeData, String> {
        match self.target {
            NodeField::NodeRef(n) => {
                let node = graph.get_node(&n)?;
                node.execute(graph)
            }
            _ => {
                return Err(format!(
                    "Error, Run target is not a Node Reference: Node {}",
                    self.name
                ));
            }
        }
    }
}
