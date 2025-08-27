use std::collections::HashMap;

use crate::node_data::NodeData;

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub pattern: String,
    pub data: HashMap<String, String>,
}

impl Template {
    /// Execute the node
    pub fn execute(self) -> Result<NodeData, String> {
        let mut output = self.pattern;
        for (k, v) in self.data {
            let placeholder = format!("{{{}}}", k);
            output = output.replace(&placeholder, &v);
        }
        Ok(NodeData::String(output))
    }
}
