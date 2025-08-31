use std::{collections::HashMap, fs::read_to_string};

use serde_json::Value;

use crate::node_data::NodeData;

#[derive(Debug, Clone)]
pub struct JsonFile {
    pub name: String,
    pub file: String,
    pub outputs: HashMap<String, String>,
}

fn get_json_object(json_value: Value, path: &str) -> Result<Value, String> {
    match json_value {
        Value::Null => Ok(Value::Null),
        Value::Bool(b) => Ok(Value::Bool(b)),
        Value::Number(n) => Ok(Value::Number(n)),
        Value::String(s) => Ok(Value::String(s)),
        Value::Array(a) => Ok(Value::Array(a)),
        Value::Object(m) => {
            let Some(obj) = m.get(path) else {
                return Err("Path not found".to_string());
            };
            Ok(obj.clone())
        }
    }
}

impl JsonFile {
    /// Execute the node
    pub fn execute(&self) -> Result<NodeData, String> {
        let content = match read_to_string(self.file.clone()) {
            Ok(content) => content,
            Err(e) => return Err(format!("Failed to read {}: {}", self.file, e)),
        };

        let json_value: Value = match serde_json::from_str(&content) {
            Ok(json_value) => json_value,
            Err(e) => return Err(format!("Failed get json value of {}: {}", self.file, e)),
        };

        let path = "names";

        let names = get_json_object(json_value, path)?;

        let Value::Array(names) = names else {
            return Err("Names if not an erray".to_string());
        };

        println!("{:?}", names);

        Ok(NodeData::None())
    }
}
