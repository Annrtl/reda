use crate::{
    graph::Graph,
    node::{json_file::JsonFile, output::Output, run::Run, template::Template},
    node_data::NodeData,
};

pub mod json_file;
pub mod output;
pub mod run;
pub mod template;

/// Unit of the flow
#[derive(Debug, Clone)]
pub enum Node {
    Unresolved(String),
    JsonFile(JsonFile),
    Output(Output),
    Template(Template),
    Run(Run),
}

impl Node {
    /// Get the name field of the node as String
    pub fn get_name(self) -> String {
        match self {
            Node::Template(n) => n.name,
            Node::Output(n) => n.name,
            Node::Run(n) => n.name,
            _ => "None".to_string(),
        }
    }

    /// Execute the node
    pub fn execute(self, graph: &Graph) -> Result<NodeData, String> {
        match self {
            Node::Output(n) => n.execute(graph),
            Node::Run(n) => n.execute(graph),
            Node::Template(n) => n.execute(),
            _ => return Err(format!("Unresolved node !")),
        }
    }
}
