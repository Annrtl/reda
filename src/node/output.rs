use std::{fs::File, io::Write};

use crate::{graph::Graph, node_data::NodeData, node_field::NodeField};

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub file: String,
    pub content: NodeField,
}

impl Output {
    /// Execute the node
    pub fn execute(&self, graph: &Graph) -> Result<NodeData, String> {
        let mut file = match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.file.clone())
        {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Failed to open file `{}` of node `{}`: {}",
                    self.file, self.name, e
                ));
            }
        };

        let data: NodeData = match &self.content {
            NodeField::NodeRef(n) => {
                let node = graph.get_node(&n)?;
                node.execute(graph)?
            }
            NodeField::String(n) => NodeData::String(n.to_string()),
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
}
