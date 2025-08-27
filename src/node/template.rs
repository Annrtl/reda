use std::collections::HashMap;

use crate::node_data::NodeData;

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub pattern: String,
    pub data: HashMap<String, String>,
}

impl Template {
    pub fn execute(self) -> Result<NodeData, String> {
        let mut output = self.pattern;
        for (k, v) in self.data {
            output = output.replace(&k, &v);
        }
        Ok(NodeData::String(output))
    }
}
