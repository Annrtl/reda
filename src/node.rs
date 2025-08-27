use crate::{
    graph::Graph,
    node::{output::Output, run::Run, template::Template},
    node_data::NodeData,
};

pub mod output;
pub mod run;
pub mod template;

/// Unit of the flow
#[derive(Debug, Clone)]
pub enum Node {
    Unresolved(String),
    Template(Template),
    Output(Output),
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

    /// Exxecute the node
    pub fn execute(self) -> Result<NodeData, String> {
        match self {
            Node::Template(n) => n.execute(),
            Node::Output(n) => n.execute(),
            Node::Run(n) => n.execute(),
            _ => return Err(format!("Unresolved node !")),
        }
    }

    pub fn resolve(&mut self, graph: Graph) -> Result<(), String> {
        match self {
            Node::Output(n) => n.resolve(graph),
            Node::Run(n) => n.resolve(graph),
            _ => return Ok(()),
        }
    }
}
