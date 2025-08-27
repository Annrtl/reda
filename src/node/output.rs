use std::{fs::File, io::Write};

use crate::{graph::Graph, node::Node, node_data::NodeData, node_field::NodeField};

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub file: String,
    pub content: Box<NodeField>,
}

impl Output {
    pub fn execute(self) -> Result<NodeData, String> {
        let mut file = match File::open(self.file) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };

        let data: NodeData = match *self.content {
            NodeField::NodeRef(n) => match n {
                crate::node::Node::Template(template) => template.execute()?,
                _ => return Err(format!("Found unappropriated type")),
            },
            NodeField::String(n) => NodeData::String(n),
            // _ => return Err(format!("Found unappropriated type"))
        };

        let content = match data {
            NodeData::String(s) => s,
            _ => return Err(format!("NodeData::String type not found")),
        };

        match file.write_all(content.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to wite file: {}", e)),
        };

        Ok(NodeData::None())
    }

    pub fn resolve(&mut self, graph: Graph) -> Result<(), String> {
        // Look for unresolved node and get its name else result
        let name = match &(*self.content) {
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
        self.content = Box::new(NodeField::NodeRef(node.clone()));

        Ok(())
    }
}
